use bevy::{
    app::Plugin,
    prelude::*,
};
use bevy_infinite_grid::{InfiniteGrid, InfiniteGridBundle, InfiniteGridPlugin};

#[derive(Default)]
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(InfiniteGridPlugin)
            .add_startup_system(setup_infinite_grid);
    }

}

fn setup_infinite_grid(
    mut commands: Commands,
) {
    commands.spawn_bundle(InfiniteGridBundle {
        grid: InfiniteGrid {
            shadow_color: Some(Color::RED),
            ..Default::default()
        },
        ..Default::default()
    });
}