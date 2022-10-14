//! Renders the current x11 display's root window in a 3d environment.

use std::{
    convert::TryInto,
    thread,
    time::Duration,
    ptr,
};


use bevy::{
    prelude::*,
    render::{
        texture::Image,
        render_resource::{
            Extent3d,
            TextureDimension,
            TextureFormat,
        },
    },
    window::PresentMode,
};

use crossbeam_channel;

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use x11::{
    xlib,
    xcomposite,
};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "virtual desktop".to_string(),
            width: 1280.0,  // 720p
            height: 720.0,
            present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_event::<WindowUpdateEvent>()
        .add_startup_system(setup_camera)
        .add_startup_system(setup_compositor)
        .add_system(process_compositor_events)
        .add_system(update_window)
        .run();
}

fn setup_camera(
    mut commands: Commands,
) {
    // light
    commands
        .spawn_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 150000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 2000.0),
            ..default()
        });

    // camera
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1900.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        });
}

fn setup_compositor(
    windows: ResMut<Windows>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Insert test screen object into world.
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad {
                size: Vec2 { x: 1920.0, y: 1080.0 },
                flip: false,
            })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_rotation(Quat::from_rotation_x(0.0f32.to_radians()))
                .with_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        })
        .insert(Window {});


    let renderer_window = unsafe {
        let raw_window = windows
            .get_primary()
            .expect("Failed to get primary 3d window.")
            .raw_window_handle()
            .get_handle()
            .raw_window_handle();

        match raw_window {
            RawWindowHandle::Xlib(handle) => handle.window,
            _ => panic!("Unsupported window type.")
        }
    };

    println!("renderer_window: {}", renderer_window);

    // Create thread to handle compositor stuff and pass events.
    let (tx, rx) = crossbeam_channel::bounded::<WindowUpdate>(1);

    thread::spawn(move || {
        // Here be (xlib) dragons...
        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());
            let root_window = xlib::XDefaultRootWindow(display);

            let overlay_window = xcomposite::XCompositeGetOverlayWindow(display, root_window);
            xlib::XReparentWindow(display, renderer_window, overlay_window, 0, 0);

            xcomposite::XCompositeRedirectSubwindows(display, root_window, xcomposite::CompositeRedirectAutomatic);

            loop {

            }

            xcomposite::XCompositeUnredirectSubwindows(display, root_window, xcomposite::CompositeRedirectAutomatic);

            xlib::XReparentWindow(display, renderer_window, root_window, 0, 0);
            xcomposite::XCompositeReleaseOverlayWindow(display, overlay_window);
        };

        /*
        // Create compositor
        let compositor = Compositor::attach_to_root();

        loop {
            //let (width, height) = compositor.dimensions();
            tx.send(WindowUpdate {
                width: 0,
                height: 0,
                bytes: compositor.get_pixels(),
            })
            .expect("Failed to send window contents to renderer!");

            thread::sleep(Duration::from_secs(1));
        }
        */
    });

    commands.insert_resource(WindowUpdateReceiver(rx));
}

fn process_compositor_events(
    receiver: ResMut<WindowUpdateReceiver>,
    mut events: EventWriter<WindowUpdateEvent>
) {
    for window in receiver.try_iter() {
        events.send(WindowUpdateEvent(window));


    }
}

fn update_window(
    mut window_content_events: EventReader<WindowUpdateEvent>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&Window, &mut Transform, &mut Handle<StandardMaterial>)>,
) {
    if let Some(event) = window_content_events.iter().last() {
        for (_window, mut transform, mut material) in query.iter_mut() {
            println!("window_update {:?}", event);

            let image = Image::new(
                Extent3d {
                    width: event.width,
                    height: event.height,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                event.bytes.clone(),
                TextureFormat::Rgba32Float
            );

            //transform.scale = Vec3::new(1.0, 1.0, window.height_ratio());
            *material = materials.add(StandardMaterial {
                base_color: Color::WHITE,
                base_color_texture: Some(images.add(image)),
                ..Default::default()
            });
        }
    }
}

#[derive(Component)]
struct Window {

}

#[derive(Debug)]
struct WindowUpdate {
    width: u32,
    height: u32,
    bytes: Vec<u8>
}

impl Default for WindowUpdate {
    fn default() -> Self {
        Self { 
            width: 0,
            height: 0,
            bytes: Vec::new(),
        }
    }
}

#[derive(Deref)]
struct WindowUpdateReceiver(crossbeam_channel::Receiver<WindowUpdate>);

#[derive(Deref, Debug)]
struct WindowUpdateEvent(WindowUpdate);

//#[derive(Component)]
struct Compositor {
    window_id: u64, 
    display: *mut xlib::Display,
}

impl Compositor {
    fn attach_to_root() -> Self {
        // Quick and dirty xlib stuff, safe but needs to be better abstracted for multiple windows.
        let (display, window_id) = unsafe {
            let display = xlib::XOpenDisplay(std::ptr::null_mut());
            let window_id = xlib::XDefaultRootWindow(display);
            
            // Redirect window content to offscreen buffer.
            xcomposite::XCompositeRedirectWindow(display, window_id,
                xcomposite::CompositeRedirectAutomatic);

            (display, window_id)
        };

        Self {
            window_id,
            display,
        }
    }

    fn dimensions(&self) -> (u32, u32) {
        unsafe {
            let mut attrs: xlib::XWindowAttributes = std::mem::zeroed();
            xlib::XGetWindowAttributes(self.display, self.window_id, &mut attrs);

            (attrs.width as u32, attrs.height as u32)
        }
    }

    fn get_pixels(&self) -> Vec<u8> {
        //let (width, height) = self.dimensions();

        unsafe {
            //let pixmap = xcomposite::XCompositeNameWindowPixmap(self.display, self.window_id);
            let mut data = Vec::new();
            //let image = xlib::XGetImage(self.display, pixmap, 0, 0,
            //        width.try_into().unwrap(), height.try_into().unwrap(),
            //        xlib::XAllPlanes(), xlib::ZPixmap);

            //let data_components = 4;  // rgba
            //let data_size = width * height * data_components;

            //let mut data = Vec::new();

            //std::slice::from_raw_parts((*image).data as *mut u8, data_size.try_into().unwrap())
            //    .chunks(data_components as usize)
            //    .for_each(|chunk| {
            //        data.extend_from_slice(&(chunk[0] as f32 / u8::MAX as f32).to_le_bytes());
            //        data.extend_from_slice(&(chunk[1] as f32 / u8::MAX as f32).to_le_bytes());
            //        data.extend_from_slice(&(chunk[2] as f32 / u8::MAX as f32).to_le_bytes());
            //        data.extend_from_slice(&1.0f32.to_le_bytes());
            //    });

            //xlib::XDestroyImage(image);

            data
        }
    }
}

impl Drop for Compositor {
    fn drop(&mut self) {
        unsafe {
            println!("Exiting compositor redirection.");
            xcomposite::XCompositeUnredirectWindow(self.display,
                self.window_id, xcomposite::CompositeRedirectAutomatic);
        }
    }
}

//fn update_window_contents(
//    mut images: ResMut<Assets<Image>>,
//    mut materials: ResMut<Assets<StandardMaterial>>,
//    mut query: Query<(&Window, &mut Transform, &mut Handle<StandardMaterial>)>
//) {
//    println!("Updating windows...");
//
//    for (window, mut transform, mut material) in query.iter_mut() {
//        //println!("dimensions: {:?}, #pixels {}, pixel: {:?}", dimensions, pixels.len());
//        let image = Image::new(
//            Extent3d {
//                width,
//                height,
//                depth_or_array_layers: 1,
//            },
//            TextureDimension::D2,
//            window.get_pixels(),
//            TextureFormat::Rgba32Float
//        );
//
//        transform.scale = Vec3::new(1.0, 1.0, window.height_ratio());
//        *material = materials.add(StandardMaterial {
//            base_color: Color::WHITE,
//            base_color_texture: Some(images.add(image)),
//            ..Default::default()
//        });
//    }
//}
//
//fn setup(
//    mut commands: Commands,
//    mut meshes: ResMut<Assets<Mesh>>,
//    mut materials: ResMut<Assets<StandardMaterial>>,
//) {
//    // plane
//    commands
//        .spawn_bundle(PbrBundle {
//            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
//            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
//            ..Default::default()
//        });
//
//    // window
//    commands
//        .spawn_bundle(PbrBundle {
//            mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
//            material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
//            transform: Transform::from_rotation(Quat::from_rotation_x(90.0f32.to_radians()))
//                .with_translation(Vec3::new(0.0, 1.0, 0.0)),
//            ..Default::default()
//        })
//        .insert(Window::from_image("/home/sushi/Pictures/wallpaper.jpg"));
//
//    // light
//    commands.spawn_bundle(PointLightBundle {
//        point_light: PointLight {
//            intensity: 1500.0,
//            shadows_enabled: true,
//            ..Default::default()
//        },
//        transform: Transform::from_xyz(0.0, 3.5, 5.0)
//            .looking_at(Vec3::ZERO, Vec3::Y),
//        ..Default::default()
//    });
//
//    // camera
//    commands.spawn_bundle(PerspectiveCameraBundle {
//        transform: Transform::from_xyz(0.0, 3.5, 5.0)
//            .looking_at(Vec3::ZERO, Vec3::Y),
//        ..Default::default()
//    });
//}
