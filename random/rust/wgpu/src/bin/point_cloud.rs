extern crate kdtree;

use kdtree::KdTree;
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub struct PointCloud {
    pub max_to_display: usize,
    points: KdTree<f32, u32, &'static [f32; 3]>,
}

impl PointCloud {
    pub fn new(max_points: usize) -> Self {
        Self {
            max_to_display: max_points,
            points: KdTree::new(3),
        }
    }

    pub fn add(&mut self, point: (f32, f32, f32)) {
        self.points.add(&[point.0, point.1, point.2], 0);
        self.refresh_gpu_buffer();
    }

    pub fn upload(&self) {

    }

    //pub fn setup(device: &wgpu::Device) -> wgpu::Pipeline {

    //}

    //pub fn render(
    //    &self,
    //    render_pass: &mut wgpu::RenderPass,
    //    pipeline: &wgpu::RenderPipeline)
    //{
    //    render_pass.set_vertex_buffers(0, self.gpu_buffer.slice(..));
    //    render_pass.set_pipeline(pipeline);
    //}

    fn refresh_gpu_buffer(&mut self) {

    }
}

//fn get_device(window: &Window) -> (wgpu::Surface, wgpu::Device, wgpu::Queue) {
//    let size = window.inner_size();
//
//    // The instance is a handle to our GPU
//    // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
//    let instance = wgpu::Instance::new(wgpu::Backends::all());
//    let surface = unsafe { instance.create_surface(window) };
//
//    let (device, queue) = pollster::block_on(async {
//        let adapter = instance
//            .request_adapter(&wgpu::RequestAdapterOptions {
//                power_preference: wgpu::PowerPreference::default(),
//                compatible_surface: Some(&surface),
//                force_fallback_adapter: false,
//            })
//            .await
//            .unwrap();
//
//        let (device, queue) = adapter
//            .request_device(
//                &wgpu::DeviceDescriptor {
//                    label: None,
//                    features: wgpu::Features::empty(),
//                    limits: wgpu::Limits::default(),
//                },
//                None, // Trace path
//            )
//            .await
//            .unwrap();
//
//        let config = wgpu::SurfaceConfiguration {
//            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
//            format: surface.get_preferred_format(&adapter).unwrap(),
//            width: size.width,
//            height: size.height,
//            present_mode: wgpu::PresentMode::Fifo,
//        };
//        surface.configure(&device, &config);
//
//        (device, queue)
//    });
//
//    (surface, device, queue)
//}

fn main() {
    let mut points = PointCloud::new();

    points.add(( 0.5,  0.0, 0.0));
    points.add((-0.5,  0.0, 0.0));
    points.add(( 0.5,  0.5, 0.0));
    points.add((-0.5,  0.5, 0.0));
    points.add(( 0.5, -0.5, 0.0));
    points.add((-0.5, -0.5, 0.0));

    println!("{:?}", points.points);

    // let event_loop = EventLoop::new();
    // let window = WindowBuilder::new().build(&event_loop).unwrap();

    // let renderer = Renderer::create(&)    

    // let (_surface, device, queue) = get_device(window);

    // println!("{:?} {:?}", device, queue);

    // // State::new uses async code, so we're going to wait for it to finish
    // let mut state = pollster::block_on(State::new(&window));

    // event_loop.run(move |event, _, control_flow| {
    //     match event {
    //         Event::WindowEvent {
    //             ref event,
    //             window_id,
    //         } if window_id == window.id() => {
    //             if !state.input(event) {
    //                 match event {
    //                     WindowEvent::CloseRequested
    //                     | WindowEvent::KeyboardInput {
    //                         input:
    //                             KeyboardInput {
    //                                 state: ElementState::Pressed,
    //                                 virtual_keycode: Some(VirtualKeyCode::Escape),
    //                                 ..
    //                             },
    //                         ..
    //                     } => *control_flow = ControlFlow::Exit,
    //                     WindowEvent::Resized(physical_size) => {
    //                         state.resize(*physical_size);
    //                     }
    //                     WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
    //                         // new_inner_size is &mut so w have to dereference it twice
    //                         state.resize(**new_inner_size);
    //                     }
    //                     _ => {}
    //                 }
    //             }
    //         }
    //         Event::RedrawRequested(window_id) if window_id == window.id() => {
    //             state.update();
    //             match state.render() {
    //                 Ok(_) => {}
    //                 // Reconfigure the surface if lost
    //                 Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
    //                 // The system is out of memory, we should probably quit
    //                 Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
    //                 // All other errors (Outdated, Timeout) should be resolved by the next frame
    //                 Err(e) => eprintln!("{:?}", e),
    //             }
    //         }
    //         Event::MainEventsCleared => {
    //             // RedrawRequested will only trigger once, unless we manually
    //             // request it.
    //             window.request_redraw();
    //         }
    //         _ => {}
    //     }
    // });
}