use std::convert::TryInto;

use image::{
    DecodingResult,
    ImageDecoder,
    jpeg::JPEGDecoder,
};

use bevy::{
    render::{
        texture::Image,
        render_resource::{
            Extent3d,
            TextureDimension,
            TextureFormat,
        },
    },
    prelude::*,
};

use x11::{
    xlib,
    xcomposite,
};

fn hello_world() {
    println!("Hello world.");
}

//#[derive(Component)]
struct Window {
    id: u64, 
    display: *mut xlib::Display,
}

impl Window {
    fn root() -> Self {
        // Quick and dirty xlib stuff, safe but needs to be better abstracted for multiple windows.
        let (display, window_id) = unsafe {
            let display = xlib::XOpenDisplay(std::ptr::null_mut());
            let window_id = xlib::XDefaultRootWindow(display);
            
            // Redirect window content to offscreen buffer.
            xcomposite::XCompositeRedirectSubwindows(display, window_id,
                xcomposite::CompositeRedirectAutomatic);

            (display, window_id)
        };

        Window {
            id: window_id,
            display,
        }
    }

    fn dimensions(&self) -> (u32, u32) {
        unsafe {
            let mut attrs: xlib::XWindowAttributes = std::mem::zeroed();
            xlib::XGetWindowAttributes(self.display, self.id, &mut attrs);

            (attrs.width as u32, attrs.height as u32)
        }
    }

    fn height_ratio(&self) -> f32 {
        let (width, height) = self.dimensions();

        height as f32 / width as f32
    }

    fn get_pixels(&self) -> Vec<u8> {
        let (width, height) = self.dimensions();

        unsafe {
            let pixmap = xcomposite::XCompositeNameWindowPixmap(self.display, self.id);
            let image = xlib::XGetImage(self.display, pixmap, 0, 0,
                    width.try_into().unwrap(), height.try_into().unwrap(),
                    xlib::XAllPlanes(), xlib::ZPixmap);

            let data_components = 4;  // rgba
            let data_size = width * height * data_components;

            let mut data = Vec::new();

            std::slice::from_raw_parts((*image).data as *mut u8, data_size.try_into().unwrap())
                .chunks(data_components as usize)
                .for_each(|chunk| {
                    data.extend_from_slice(&(chunk[0] as f32 / u8::MAX as f32).to_le_bytes());
                    data.extend_from_slice(&(chunk[1] as f32 / u8::MAX as f32).to_le_bytes());
                    data.extend_from_slice(&(chunk[2] as f32 / u8::MAX as f32).to_le_bytes());
                    data.extend_from_slice(&1.0f32.to_le_bytes());
                });

            xlib::XDestroyImage(image);

            data
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

fn main() {
    //App::new()
    //    .insert_resource(Msaa { samples: 4 })
    //    .add_plugins(DefaultPlugins)
    //    .add_startup_system(hello_world)
    //    .add_startup_system(setup)
    //    .add_system(update_window_contents)
    //    //.add_startup_system(setup_dataset)
    //    .run();
    
    let window = Window::root();
    let data = window.get_pixels();
}


