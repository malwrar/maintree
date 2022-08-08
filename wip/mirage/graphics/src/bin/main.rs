//! Main 3d renderer for mirage.

use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::PrimitiveTopology,
        render_resource::{
            AsBindGroup,
            ShaderRef
        },
    }
};

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<FloorMaterial>::default())
        .add_startup_system(setup_grid)
        .add_startup_system(setup_scene)
        //.add_startup_system(setup_ui)
        .run();
}

//fn create_grid_base_mesh() -> Mesh {
//    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
//    let vertices = vec![
//        [-1.0,  1.0, 0.0],
//        [-1.0, -1.0, 0.0],
//        [ 1.0,  1.0, 0.0],
//        [ 1.0, -1.0, 0.0],
//        [ 1.0,  1.0, 0.0],
//        [-1.0, -1.0, 0.0],
//    ];
//
//
//    let normals = (0..vertices.len())
//        .map(|_| [0.0, 0.0, 1.0])
//        .collect::<Vec<[f32; 3]>>();
//
//    let uvs = (0..vertices.len())
//        .map(|_| [0.0, 1.0])
//        .collect::<Vec<[f32; 2]>>();
//
//    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
//    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
//    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
//    mesh.set_indices(None);
//    mesh
//}

fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FloorMaterial>>,
) {
    commands.spawn().insert_bundle(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 8.0 })),
        //mesh: meshes.add(create_grid_base_mesh()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: materials.add(FloorMaterial {
            color: Color::BLUE,
        }),
        ..default()
    });
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct FloorMaterial {
    #[uniform(0)]
    color: Color,
}

impl Material for FloorMaterial {
    //fn vertex_shader() -> ShaderRef {
    //    "shaders/floor.wgsl".into()
    //}

    fn fragment_shader() -> ShaderRef {
        "shaders/floor.wgsl".into()
    }
}

//fn setup_ui(
//    mut commands: Commands,
//    mut asset_server: Res<AssetServer>,
//) {
//    // Camera
//    commands.spawn_bundle(Camera2dBundle::default());
//}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.6, 0.0, 0.0).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });

    // Light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // Camera
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(5.0, 3.5, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };
    commands.spawn_bundle(camera);
}