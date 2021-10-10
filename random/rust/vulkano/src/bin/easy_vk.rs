//! An attempt to implement an easier abstraction over vulkan.
//!
//! I really like the vulkan api, but most aspects around initialization and runtime handholding
//! (re-creating the swap buffer on window resize, for example) make it tedious to reuse without
//! some sort of abstraction in between. Hopefully this'll work!
//!

#![allow(unused_imports, dead_code, unused_variables)]

#[macro_use]
extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

use std::collections::HashSet;
use std::fmt;
use std::sync::Arc;

use winit::{EventsLoop, WindowBuilder, Window, dpi::LogicalSize, Event, WindowEvent};
use vulkano_win::VkSurfaceBuild;

use vulkano::instance::{
    Instance,
    InstanceExtensions,
    ApplicationInfo,
    Version,
    layers_list,
    PhysicalDevice,
};
use vulkano::instance::debug::{DebugCallback, MessageTypes};
use vulkano::device::{Device, DeviceExtensions, Queue, Features};
use vulkano::swapchain::{
    Surface,
    Capabilities,
    ColorSpace,
    SupportedPresentModes,
    PresentMode,
    Swapchain,
    CompositeAlpha,
    acquire_next_image,
    AcquireError,
};
use vulkano::format::Format;
use vulkano::image::{ImageUsage, swapchain::SwapchainImage};
use vulkano::sync::{self, SharingMode, GpuFuture};
use vulkano::pipeline::{
    GraphicsPipeline,
    GraphicsPipelineAbstract,
    viewport::Viewport,
};
use vulkano::framebuffer::{
    RenderPassAbstract,
    Subpass,
    FramebufferAbstract,
    Framebuffer,
};
use vulkano::command_buffer::{
    AutoCommandBuffer,
    AutoCommandBufferBuilder,
    DynamicState,
};
use vulkano::buffer::{
    immutable::ImmutableBuffer,
    BufferUsage,
    BufferAccess,
    TypedBufferAccess,
};

//#[cfg(all(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = true;
//#[cfg(not(debug_assertions))]
//const ENABLE_VALIDATION_LAYERS: bool = false;

const VALIDATION_LAYERS: &[&str] =  &[
    "VK_LAYER_LUNARG_standard_validation"
];

/// Required device extensions
fn device_extensions() -> DeviceExtensions {
    DeviceExtensions {
        khr_swapchain: true,
        .. vulkano::device::DeviceExtensions::none()
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    pos: [f32; 2],
    color: [f32; 3],
}
impl Vertex {
    fn new(pos: [f32; 2], color: [f32; 3]) -> Self {
        Self { pos, color }
    }
}
impl_vertex!(Vertex, pos, color);

fn vertices() -> [Vertex; 4] {
    [
        Vertex::new([-0.5, -0.5], [1.0, 0.0, 0.0]),
        Vertex::new([0.5, -0.5], [0.0, 1.0, 0.0]),
        Vertex::new([0.5, 0.5], [0.0, 0.0, 1.0]),
        Vertex::new([-0.5, 0.5], [1.0, 1.0, 1.0])
    ]
}

fn indices() -> [u16; 6] {
    [0, 1, 2, 2, 3, 0]
}

/// Convenient wrapper over vulkan instance operations.
#[derive(Clone, Debug)]
struct VkInstance {
    native_instance: Arc<Instance>,
}

impl VkInstance {
    pub fn new() -> Self {
        let supported_extensions = InstanceExtensions::supported_by_core()
            .expect("Failed to retrieve supported extensions!");
        log::debug!("Supported extensions: {:?}", supported_extensions);

        let app_info = ApplicationInfo {
            application_name: Some("vulkan tutorial".into()),
            application_version: Some(Version { major: 1, minor: 0, patch: 0 }),
            engine_name: Some("n/a".into()),
            engine_version: Some(Version { major: 1, minor: 0, patch: 0 }),
        };

        let required_extensions = Self::get_required_vk_extensions();

        let native_instance = if Self::check_vk_validation_layer_support() && ENABLE_VALIDATION_LAYERS {
            Instance::new(Some(&app_info), &required_extensions,
                    VALIDATION_LAYERS.iter().cloned())
        } else {
            Instance::new(Some(&app_info), &required_extensions, None)
        }.expect("Failed to create Vulkan instance!");

        Self {
            native_instance,
        }
    }

    pub fn physical_device_indices(&self) -> Vec<i32> {
        // Working with device indices is easier than handling the PhysicalDevice obj
        (0..PhysicalDevice::enumerate(&self.native_instance).len())
            .map(|x| x as i32)
            .collect()
    }

    fn check_vk_validation_layer_support() -> bool {
        let layers: Vec<_> = layers_list().unwrap().map(|l| l.name().to_owned()).collect();
        VALIDATION_LAYERS.iter()
            .all(|layer_name| layers.contains(&layer_name.to_string()))
    }

    fn get_required_vk_extensions() -> InstanceExtensions {
        let mut extensions = vulkano_win::required_extensions();
        if ENABLE_VALIDATION_LAYERS {
            // TODO!: this should be ext_debug_utils (_report is deprecated), but that doesn't exist yet in vulkano
            extensions.ext_debug_report = true;
        }

        extensions
    }
}

/// Combines vulkans physical device with its logical device constructs.
#[derive(Clone, Debug)]
struct VkDevice {
    instance: VkInstance,
    surface: Arc<Surface<winit::Window>>,
    physical_device_index: usize,
    logical_device: Arc<Device>, 
    graphics_queue: Arc<Queue>,
    present_queue: Arc<Queue>,
}

impl VkDevice {
    pub fn from_index(
        instance: &VkInstance,
        surface: &Arc<Surface<winit::Window>>,
        device_index: usize
    ) -> Option<Self> {
        let physical_device = PhysicalDevice::from_index(&instance.native_instance, device_index).unwrap();

        // Find queue families
        let mut graphics_family: i32 = -1;
        let mut present_family: i32 = -1;
        
        for (i, queue_family) in physical_device.queue_families().enumerate() {
            if queue_family.supports_graphics() {
                graphics_family = i as i32;
            }

            if surface.is_supported(queue_family).unwrap() {
                present_family = i as i32;
            }
        }

        if graphics_family < 0 || present_family < 0 {
            return None;
        }

        let families = [graphics_family, present_family];
        use std::iter::FromIterator;
        let unique_queue_families: HashSet<&i32> = HashSet::from_iter(families.iter());

        let queue_priority = 1.0;
        let queue_families = unique_queue_families.iter().map(|i| {
            (physical_device.queue_families().nth(**i as usize).unwrap(), queue_priority)
        });

        let (logical_device, mut queues) = Device::new(physical_device, &Features::none(),
            &device_extensions(), queue_families)
            .expect("failed to create logical device!");

        let graphics_queue = queues.next().unwrap();
        let present_queue = queues.next().unwrap_or_else(|| graphics_queue.clone());

        Some(Self {
            instance: instance.clone(),
            surface: surface.clone(),
            physical_device_index: device_index,
            logical_device,
            graphics_queue,
            present_queue
        })
    }

    pub fn find_compatible(
        instance: &VkInstance,
        surface: &Arc<Surface<winit::Window>>,
    ) -> Vec<Self> {
        let mut candidates = Vec::new();

        for idx in instance.physical_device_indices() {
            let device = match Self::from_index(instance, surface, idx as usize) {
                Some(device) => device,
                None => continue
            };
            candidates.push(device);
        }

        candidates
    }

    pub fn find_best(
        instance: &VkInstance,
        surface: &Arc<Surface<winit::Window>>,
    ) -> Option<Self> {
        let mut candidates = Self::find_compatible(instance, surface);

        // TODO: rate these somehow? for now though, who cares
        candidates.pop()
    }

    fn physical_device(&self) -> PhysicalDevice {
        PhysicalDevice::from_index(&self.instance.native_instance, self.physical_device_index).unwrap()
    }
}

///
struct VkSwapchain {
    device: VkDevice,
    surface: Arc<Surface<winit::Window>>,
    format: Format,
    colorspace: ColorSpace,
    present_mode: PresentMode,
    swapchain: Arc<Swapchain<Window>>,
    images: Vec<Arc<SwapchainImage<Window>>>,
    x: u32,
    y: u32,
}

impl VkSwapchain {
    fn new(
        device: &VkDevice,
        surface: Arc<Surface<winit::Window>>,
        x: u32,
        y: u32,
    ) -> Self {
      let capabilities = surface.capabilities(device.physical_device())
            .expect("failed to get surface capabilities");

        let (format, colorspace) = Self::choose_swap_surface_format(&capabilities.supported_formats);
        let present_mode = Self::choose_swap_present_mode(capabilities.present_modes);
        let extent = Self::choose_swap_extent(&capabilities, x, y);

        let mut image_count = capabilities.min_image_count + 1;
        if capabilities.max_image_count.is_some()
                && image_count > capabilities.max_image_count.unwrap() {
            image_count = capabilities.max_image_count.unwrap();
        }

        let image_usage = ImageUsage {
            color_attachment: true,
            .. ImageUsage::none()
        };

        let (swapchain, images) = Swapchain::new(
            device.logical_device.clone(),
            surface.clone(),
            image_count,
            format,
            extent,
            1, // layers
            image_usage,
            &device.graphics_queue,
            capabilities.current_transform,
            CompositeAlpha::Opaque,
            present_mode,
            true, // clipped
            None //old_swapchain.as_ref()
        ).expect("Failed to create swap chain!");

        Self {
            device: device.clone(),
            surface,
            format,
            colorspace,
            present_mode,
            swapchain,
            images,
            x,
            y,
        }
    }

    fn choose_swap_surface_format(
        available_formats: &[(Format, ColorSpace)]
    ) -> (Format, ColorSpace) {
        // NOTE: the 'preferred format' mentioned in the tutorial doesn't seem to be
        // queryable in Vulkano (no VK_FORMAT_UNDEFINED enum)
        *available_formats.iter()
            .find(|(format, color_space)|
                *format == Format::B8G8R8A8Unorm && *color_space == ColorSpace::SrgbNonLinear
            )
            .unwrap_or_else(|| &available_formats[0])
    }

    fn choose_swap_present_mode(available_present_modes: SupportedPresentModes) -> PresentMode {
        if available_present_modes.mailbox {
            PresentMode::Mailbox
        } else if available_present_modes.immediate {
            PresentMode::Immediate
        } else {
            PresentMode::Fifo
        }
    }

    fn choose_swap_extent(
        capabilities: &Capabilities,
        x: u32,
        y: u32,
    ) -> [u32; 2] {
        if let Some(current_extent) = capabilities.current_extent {
            return current_extent;
        } else {
            let mut actual_extent = [x, y];
            actual_extent[0] = capabilities.min_image_extent[0]
                .max(capabilities.max_image_extent[0].min(actual_extent[0]));
            actual_extent[1] = capabilities.min_image_extent[1]
                .max(capabilities.max_image_extent[1].min(actual_extent[1]));
            actual_extent
        }
    }
}

/// Native OS window render target.
struct VkWindow {
    instance: VkInstance,
    event_loop: EventsLoop,
    surface: Arc<Surface<winit::Window>>,
    device: VkDevice,
    swapchain: VkSwapchain,
}

impl VkWindow {
    fn new(instance: &VkInstance, width: u32, height: u32) -> Self {
        let event_loop = EventsLoop::new();
        let surface = WindowBuilder::new()
            .with_title("Vulkan")
            .with_dimensions(LogicalSize::new(f64::from(width), f64::from(height)))
            .build_vk_surface(&event_loop, instance.native_instance.clone())
            .expect("Failed to create window surface!");

        let device = VkDevice::find_best(&instance, &surface)
            .expect("Failed to find device that supports rendering with Vulkan.");

        let swapchain = VkSwapchain::new(&device, surface.clone(), width, height);

        Self {
            instance: instance.clone(),
            event_loop,
            surface,
            device,
            swapchain,
        }
    }

    fn poll_events<F>(&mut self, event_handler: F) where
            F: Fn(Event) {
        self.event_loop.poll_events(event_handler);
    }

    fn add_graphics_pipeline(&mut self) {

    }
}

fn create_render_pass(device: &Arc<Device>, color_format: Format) -> Arc<RenderPassAbstract + Send + Sync> {
    Arc::new(single_pass_renderpass!(device.clone(),
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: color_format,
                samples: 1,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {}
        }
    ).unwrap())
}

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();

    log::info!("Starting up...");
    log::warn!("Starting up...");
    log::debug!("Starting up...");
    log::error!("Starting up...");

    log::debug!("Creating vk instance...");
    let instance = VkInstance::new();

    log::debug!("Spawning vk window...");
    let mut window = VkWindow::new(&instance, 640, 480);

    log::debug!("Loading shaders...");
    mod vertex_shader {
        vulkano_shaders::shader! {
           ty: "vertex",
           path: "src/shaders/triangle.vert"
        }
    }

    mod fragment_shader {
        vulkano_shaders::shader! {
            ty: "fragment",
           path: "src/shaders/triangle.frag"
        }
    }

    let vert_shader_module = vertex_shader::Shader::load(window.device.logical_device.clone())
        .expect("failed to create vertex shader module!");
    let frag_shader_module = fragment_shader::Shader::load(window.device.logical_device.clone())
        .expect("failed to create fragment shader module!");

    log::debug!("Creating vertex buffer...");
    let dimensions = [
        window.swapchain.x as f32,
        window.swapchain.y as f32
    ];

    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions,
        depth_range: 0.0 .. 1.0,
    };

    let render_pass = create_render_pass(&window.device.logical_device,
            window.swapchain.format);

    let pipeline = Arc::new(GraphicsPipeline::start()
        .vertex_input_single_buffer::<Vertex>()
        .vertex_shader(vert_shader_module.main_entry_point(), ())
        .triangle_list()
        .primitive_restart(false)
        .viewports(vec![viewport]) // NOTE: also sets scissor to cover whole viewport
        .fragment_shader(frag_shader_module.main_entry_point(), ())
        .depth_clamp(false)
        // NOTE: there's an outcommented .rasterizer_discard() in Vulkano...
        .polygon_mode_fill() // = default
        .line_width(1.0) // = default
        .cull_mode_back()
        .front_face_clockwise()
        // NOTE: no depth_bias here, but on pipeline::raster::Rasterization
        .blend_pass_through() // = default
        .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
        .build(window.device.logical_device.clone())
        .unwrap());

    loop {
        window.poll_events(|event| {
            log::debug!("event: {:?}", event);
            //if should refresh {
            //    window.draw()
            //}
        })
    }
}


// 3dge == 3d graphics engine (get it?)
// basically, you give it object(s) to render, viewport(s) to view objects on,
// scripts that add dynamic elements based on bpf-specified engine event filters + lua, and controllers
// that generate events like wasd (raw keys) pull up/down/left/right (0.0 -> 1.0 intensity), pitch
// up/down, yaw
// effectively, we give the engine objects, viewports, and controllers for interacting with
// objects. we can also give it lua scripts that define
//
// everything is an object, rendered based on proximity to the viewport(s)
//   * viewports can be
//     - windows
//     - virtual cameras that render to video containers (even offline, for really thicc renders)
//     - vr headsets
//     - blind-accessible (audio cues, some sort of gesture command
//       system/probe to interrogate environment using the leap or gloves or
//       whatever, sign language interpreter, generally weave spells as a control mechanism)
//     etc
//
// this allows the user to
//   * ask for a hands-off viewport as an augmentation to println!(),
//   * explore datasets in 3d space
//   * create a client experience, essentially 3d websites
//     * edge://site.com
//       * serves compressed main.lua
//       * backwards compatible with http
//   * create video games
//   * perform intensive offline rendering, simulation, etc
//
// all the platform needs to do is provide a the following
//   * viewport - method of visualizing, or other percieving, the environment
//     * embedded popup window in rust program, created by macro
//     * specialized window that implements a video game experience
//     * vr headset that represents a physical actor in the space
//     * ar headset that attempts to project objects into a real space with backwards vr
//       compatability attempted
//     * virtual video camera
//     * bot that just asks for raw data that doesn't necessarily need to be be video
//       * point cloud
//       * ray traces
//       * surfaces
//       * maps
//       * etc
//   * controller - method of interacting with the objects, environment, & system
//     * keyboard and mouse
//     * voice
//     * gesture
//     * joystick
//     * virtual car controls
//     * piano
//     * vr wand device
//  * graphics processor
//     * basically anything that implements the vulkan interfaces we need
//       * maybe switch to rendy later so we can work in browsers
//  * asset storage
//     * place to put big things that won't change often, like video, textures, images, etc
//  * state storage
//     * place to put temporary data, like entity positions, scores, etc
// 
// as hinted at previously, it would be cool if people could distribute more curated experiences,
// like websites, that get rendered in this engine. lua can be used with bpf to create pretty much
// any experience. it can even capture things like keyboard events and use them to generate useful
// control signals that can be shared universally to anyone who subscribes to those sorts of
// events. the whole thing is one big open ecosystem of lua packages distributed over a custom sync
// service or over legacy http.
//
//
// read lua packages from edge://site.com sites (server we provide that REALLY efficiently serves
// compressed lua packages w/ all assets)
// advantages are numerous:
//
//let edge = EdgeWindow()
//    .float()
//    .background_render(|event| {
//        log::debug("Events from engine thread, event");
//        // alternative is to not use this and get a poll_events fn to run in a loop
//    });
//    .spawn();
//// !edge_viewport([ EdgeWindow, EdgeVr([ "ValveIndex" ]), EdgeAr([ "
//
//edge.add_pointcloud();
//edge.add_shape();
//edge.add_scripted_entity();
//// basically, add_xxx() where xxx is some object/surface/item/force/etc in the world. users can
//// implement their own types and create whatever they want while trusting us to render it
//// efficiently. types can be implemented in rust or lua.
//
//fn 
//
//edge.add_controller();
//
//let camera = edge.viewport()
//
//edge.background_render(
//    Some(|event| {
//        log::debug("event in another thread woooo: {:?}, event");
//    }))
//    .fps(
