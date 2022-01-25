//! The obligatory graphics api hello world.

use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use edge::{Mesh, Renderer}

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Warn)
        .filter_module("less_fuss", log::LevelFilter::Debug)
        .init();

    log::info!("Starting up...");
    log::warn!("Starting up...");
    log::debug!("Starting up...");
    log::error!("Starting up...");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(960, 640))
        .with_resizable(false)
        .with_title("Test")
        .with_decorations(true)
        .build(&event_loop)
        .unwrap();

    let mesh = Mesh::new()
        .vertices(&[
            ( 0.0,  0.5, 0.0),
            (-0.5, -0.5, 0.0),
            ( 0.5, -0.5, 0.0),
        ])
        .build();

    let renderer = Renderer::on_surface(Surface)
    renderer.draw(&mesh);

    loop {}
}