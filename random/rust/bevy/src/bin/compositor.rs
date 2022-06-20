use std::fs;

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

//use x11::{
//    xlib,
//    xcomposite,
//};

fn hello_world() {
    println!("Hello world.");
}

#[derive(Component)]
struct Window {
    pub path: String, 
}

impl Window {
    fn from_image(path: &str) -> Self {
        Window {
            path: String::from(path),
        }
    }

    fn height_ratio() -> f32 {
        1.0
    }
}

fn update_window_contents(
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&Window, &mut Transform, &mut Handle<StandardMaterial>)>
) {
    println!("Updating windows...");

    for (window, mut transform, mut material) in query.iter_mut() {
        let file = fs::File::open(&window.path).unwrap();
        let mut jpeg = JPEGDecoder::new(file);
        let (width, height) = jpeg.dimensions().unwrap();

        let pixels = match jpeg.read_image().unwrap() {
            DecodingResult::U8(data) => data,
            _ => panic!("Image uses unsupported format.")
        };

        // Convert data from rgb to rgba. We assume `pixels` vec takes the form
        // [ r, g, b, ... , r, g, b ].
        let mut rgba_pixels = Vec::new();

        for chunk in pixels.chunks(3) {
            rgba_pixels.extend_from_slice(&(chunk[0] as f32 / u8::MAX as f32).to_le_bytes());
            rgba_pixels.extend_from_slice(&(chunk[1] as f32 / u8::MAX as f32).to_le_bytes());
            rgba_pixels.extend_from_slice(&(chunk[2] as f32 / u8::MAX as f32).to_le_bytes());
            rgba_pixels.extend_from_slice(&1.0f32.to_le_bytes());
        }

        //println!("dimensions: {:?}, #pixels {}, pixel: {:?}", dimensions, pixels.len());
        let image = Image::new(
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            rgba_pixels,
            TextureFormat::Rgba32Float
        );

        transform.scale = Vec3::new(1.0, 1.0, height as f32 / width as f32);
        *material = materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(images.add(image)),
            ..Default::default()
        });
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        });

    // window
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
            material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
            transform: Transform::from_rotation(Quat::from_rotation_x(90.0f32.to_radians()))
                .with_translation(Vec3::new(0.0, 1.0, 0.0)),
            ..Default::default()
        })
        .insert(Window::from_image("/home/sushi/Pictures/wallpaper.jpg"));

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 3.5, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 3.5, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(hello_world)
        .add_startup_system(setup)
        .add_system(update_window_contents)
        //.add_startup_system(setup_dataset)
        .run();
}


