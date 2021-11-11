use std::iter;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use wgpu::util::DeviceExt;

struct EzSurface {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub clear_color: (f64, f64, f64, f64),

    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
}

impl EzSurface {
    fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .enumerate_adapters(wgpu::Backends::all())
            .filter(|adapter| {
                // Check if this adapter supports our surface
                surface.get_preferred_format(&adapter).is_some()
            })
            .next()
            .unwrap();

        
        let (device, queue) = pollster::block_on(async {
            adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        label: None,
                        features: wgpu::Features::empty(),
                        limits: wgpu::Limits::default(),
                    },
                    // Some(&std::path::Path::new("trace")), // Trace path
                    None,
                )
                .await
                .unwrap()
        });

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let mut surface = Self {
            surface,
            surface_config,
            device,
            queue,
            clear_color: (1.0, 0.1, 0.1, 1.0),
        };
        surface.reconfigure();  // Technically the first time it'll be configured, but who's counting?

        surface
    }

    fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    fn reconfigure(&mut self) {
        self.resize(self.surface_config.width, self.surface_config.height);
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_frame()?.output;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("render_encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render_pass"),
                color_attachments: &[
                    wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: self.clear_color.0,
                                g: self.clear_color.1,
                                b: self.clear_color.2,
                                a: self.clear_color.3,
                            }),
                            store: true,
                        },
                    }
                ],
                depth_stencil_attachment: None,
            });

            // TODO: draw (scenes full of?) objects with pipelines efficiently!
        }

        self.queue.submit(iter::once(encoder.finish()));

        Ok(())
    }
}

struct EzBakedPipeline {
}

struct EzPipeline {
}

impl EzPipeline {
    fn new() -> EzPipeline {
        EzPipeline::default()
    }

    fn bake_for(surface: &EzSurface) -> EzBakedPipeline {
        EzBakedPipeline {

        }
    }
}

impl Default for EzPipeline {
    fn default() -> Self {
        Self {

        }
    }
}

struct EzMesh {

}

trait Drawable {
    fn mesh() -> EzMesh {
        EzMesh {

        }
    }

    fn pipeline() -> EzPipeline {
        EzPipeline::new()
    }
}

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();

    log::info!("Starting up...");
    log::warn!("Starting up...");
    log::debug!("Starting up...");
    log::error!("Starting up...");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    let mut surface = EzSurface::new(&window);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        surface.resize(physical_size.width, physical_size.height);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &&mut so w have to dereference it twice
                        surface.resize(new_inner_size.width, new_inner_size.height);
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(_) => {
                log::debug!("RedrawRequested");
                match surface.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => surface.reconfigure(),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => log::error!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // Keep the redraws coming!
                window.request_redraw();
            }
            _ => {}
        }
    });
}
