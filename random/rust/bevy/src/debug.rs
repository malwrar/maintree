use bevy::{
    app::Plugin,
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::*
};
use bevy_infinite_grid::{InfiniteGrid, InfiniteGridBundle, InfiniteGridPlugin};

#[derive(Default)]
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(MaterialPlugin::<GridMaterial>::default())
            .add_plugin(InfiniteGridPlugin)
            .add_startup_system(setup_infinite_grid);
    }

}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f477b6f5-dcf8-4774-b54a-adeb13be791c"]
struct GridMaterial {}

impl Material for GridMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/infinite_grid.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/infinite_grid.wgsl".into()
    }
}

fn setup_infinite_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GridMaterial>>,
) {
    commands.spawn_bundle(InfiniteGridBundle {
        grid: InfiniteGrid {
            shadow_color: None,
            ..Default::default()
        },
        ..Default::default()
    });
 
    /*
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(1., 1., 1.),
            ..Default::default()
        })
        .insert(FlyCam)
        .insert(GridShadowCamera);
    */
}