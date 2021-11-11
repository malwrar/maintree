use std::iter;

use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use wgpu::util::DeviceExt;

struct EzSurface {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,

    pub native: wgpu::Surface,
    pub native_config: wgpu::SurfaceConfiguration,

    pub clear_color: (f64, f64, f64, f64),
}

impl EzSurface {
    fn new(window: &Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let native = unsafe { instance.create_surface(window) };

        // Ensure we can draw to the surface.
        let adapter = instance
            .enumerate_adapters(wgpu::Backends::all())
            .filter(|adapter| {
                native.get_preferred_format(&adapter).is_some()
            })
            .next()
            .unwrap();

        
        // Choose optimal device.
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

        let native_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: native.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let mut surface = Self {
            native,
            native_config,
            device,
            queue,
            clear_color: (1.0, 0.1, 0.1, 1.0),
        };
        surface.reconfigure();  // Technically the first time it'll be configured, but who's counting?

        surface
    }

    fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.native_config.width = width;
            self.native_config.height = height;
            self.native.configure(&self.device, &self.native_config);
        }
    }

    fn reconfigure(&mut self) {
        self.resize(self.native_config.width, self.native_config.height);
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let thingy = Thingy {};

        let mesh = match thingy.mesh() {
            Some(mesh) => mesh.bake(&self.device),
            None => panic!("No default mesh!"),
        };
        let pipeline = Thingy::render_pipeline()
            .bake_for(self);

        let output = self.native.get_current_frame()?.output;
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

            render_pass.set_pipeline(&pipeline.native);
            //render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));

            if let Some(index_buffer) = &mesh.index_buffer {
                render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..mesh.num_indices, 0, 0..1);
            }

            // TODO: draw (scenes full of?) objects with pipelines efficiently!
        }

        self.queue.submit(iter::once(encoder.finish()));

        Ok(())
    }
}

struct EzBakedRenderPipeline {
    native: wgpu::RenderPipeline,
}

struct EzRenderPipeline {
}

impl EzRenderPipeline {
    fn new() -> EzRenderPipeline {
        EzRenderPipeline::default()
    }

    fn bake_for(&self, surface: &EzSurface) -> EzBakedRenderPipeline {
        let shader = surface.device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/default.wgsl").into()),
        });

        let render_pipeline_layout =
            surface.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("render_pipeline_layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = surface.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render_pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "main",
                buffers: &[
                    Vertex::desc(),
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: surface.native_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                clamp_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        EzBakedRenderPipeline {
            native: render_pipeline,
        }
    }
}

impl Default for EzRenderPipeline {
    fn default() -> Self {
        Self {
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        }
    }
}

struct CpuMesh {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}

impl CpuMesh {
    fn new(
        vertices: Vec<Vertex>,
        indices: Vec<u16>,
    ) -> Self {
        Self {
            vertices,
            indices,
        }
    }

    fn bake(&self, device: &wgpu::Device) -> GpuMesh {
        GpuMesh::new(device, self.vertices.as_slice(), self.indices.as_slice())
    }
}

struct GpuMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: Option<wgpu::Buffer>,
    pub num_indices: u32,
}

impl GpuMesh {
    fn new(
        device: &wgpu::Device,
        vertices: &[Vertex],
        indices: &[u16],
    ) -> Self {
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("vertex_buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let num_indices = indices.len() as u32;
        let index_buffer = if num_indices > 0 {
            Some(device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("index_buffer"),
                    contents: bytemuck::cast_slice(indices),
                    usage: wgpu::BufferUsages::INDEX,
                }
            ))
        } else {
            None
        };

        Self {
            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }
}

trait Object {
    fn mesh(&self) -> Option<CpuMesh> { None }
    fn compute_pipeline() -> Option<EzRenderPipeline> { None }
    fn render_pipeline() -> EzRenderPipeline { EzRenderPipeline::default() }
}

struct Thingy {

}

impl Object for Thingy {
    fn mesh(&self) -> Option<CpuMesh> {
        let mesh = CpuMesh::new(vec![
                Vertex { position: [-0.0868241,   0.49240386, 0.0], color: [0.5, 0.3, 0.5] }, // A
                Vertex { position: [-0.49513406,  0.06958647, 0.0], color: [0.1, 0.5, 0.5] }, // B
                Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.3, 1.0, 0.5] }, // C
                Vertex { position: [ 0.35966998, -0.3473291,  0.0],  color: [0.9, 0.4, 0.5] }, // D
                Vertex { position: [ 0.44147372,  0.2347359,  0.0],  color: [0.2, 0.3, 0.5] }, // E
            ], vec![
                0, 1, 4,
                1, 2, 4,
                2, 3, 4,
                /* padding */ 0,
            ]);

        Some(mesh)
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
        .with_inner_size(PhysicalSize::new(960, 640))
        .with_resizable(false)
        .with_title("Test")
        .with_decorations(true)
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
