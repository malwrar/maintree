//! Contains code written while following https://vulkan-tutorial.com
//!
//! Tutorial followed can be found here: https://vulkan-tutorial.com/
//! Many references made to https://github.com/bwasty/vulkan-tutorial-rs for shortcuts on
//! vulkano-specific abi usage.

#![allow(unused_imports)]

use std::sync::Arc;
use std::collections::HashSet;

use winit::{EventsLoop, WindowBuilder, Window, dpi::LogicalSize, Event, WindowEvent};
use vulkano_win::VkSurfaceBuild;

use priority_queue::PriorityQueue;

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
};
use vulkano::format::Format;
use vulkano::image::{ImageUsage, swapchain::SwapchainImage};
use vulkano::sync::SharingMode

use vulkano::instance::debug::{};

//#[cfg(all(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = true;
//#[cfg(not(debug_assertions))]
//const ENABLE_VALIDATION_LAYERS: bool = false;

const VALIDATION_LAYERS: &[&str] =  &[
    "VK_LAYER_LUNARG_standard_validation"
];

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

/// Contains stuff needed to create & run a vulkan rendering window.
struct TutorialApp {
    events_loop: EventsLoop,
    debug_callback: Option<DebugCallback>,
    instance: Arc<Instance>,
    physical_device_index: usize,
}

impl TutorialApp {
    pub fn new() -> Self {
        let instance = Self::create_vk_instance();
        log::info!("Created VK instance.");

        let (events_loop, surface) = Self::create_vk_surface(&instance);
        log::info!("Spawned window.");

        let debug_callback = Self::setup_vk_debug_callback(&instance);
        log::info!("Installed validation layer debug callback.");

        let physical_device_index = Self::pick_vk_physical_device(&instance);
        log::info!("Selected suitable vk physical device.");

        Self {
            events_loop,
            debug_callback,
            instance,
            physical_device_index,
        }
    }

    /// Step 1: create the core vulkan api context container.
    fn create_vk_instance() -> Arc<Instance> {
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

        if Self::check_vk_validation_layer_support() && ENABLE_VALIDATION_LAYERS {
            Instance::new(Some(&app_info), &required_extensions,
                    VALIDATION_LAYERS.iter().cloned())
        } else {
            Instance::new(Some(&app_info), &required_extensions, None)
        }.expect("Failed to create Vulkan instance!")  // NOTE: Instance::new() returns a Result that should be handled the same way in both instances (lol), do so here
    }

    /// Step 2: create the window.
    fn create_vk_surface(instance: &Arc<Instance>) -> (EventsLoop, Arc<Surface<Window>>) {
        let events_loop = EventsLoop::new();
        let _window = WindowBuilder::new()
            .with_title("Vulkan")
            .with_dimensions(LogicalSize::new(f64::from(WIDTH), f64::from(HEIGHT)))
            .build(&events_loop);
        events_loop
    }

    /// Step 3: tell vulkan to yell at me if it thinks i fucked something up.
    fn setup_vk_debug_callback(instance: &Arc<Instance>) -> Option<DebugCallback> {
        if !ENABLE_VALIDATION_LAYERS  {
            return None;
        }

        let msg_types = MessageTypes {
            error: true,
            warning: true,
            performance_warning: true,
            information: false,
            debug: true,
        };
        DebugCallback::new(&instance, msg_types, |msg| {
            log::warn!("Validation layer: {:?}", msg.description);
        }).ok()
    }

    /// Step 4: figure out what lucky gpu gets to process our fancy linear algebra for us
    fn pick_vk_physical_device(instance: &Arc<Instance>) -> usize {
        let mut pq = PriorityQueue::new();
        for candidate in PhysicalDevice::enumerate(&instance) {
            candidate_score = 0.0f32;

            
        }
    }

    /// Determine if the given PhysicalDevice can do everything we need it to do.
    fn is_vk_device_suitable(device: &PhysicalDevice) -> bool {
        // For now, we just care about whether or not the device supports graphics.
        Self::find_vk_device_queue_index(device, true).is_some()
    }

    /// Find the index of a queue family that has all the features requested by the caller.
    fn find_vk_device_queue_index(
        device: &PhysicalDevice,
        graphics_support_required: bool,
    ) -> Option<usize> {
        // TODO: replace index with id to simplify?

        let mut idx: Option<usize> = None;

        log::debug!("| + queue_families:");
        for (candidate_idx, queue_family) in device.queue_families().enumerate() {
            log::debug!("|   - {:04} {:?}", candidate_idx, queue_family);
            if graphics_support_required && queue_family.supports_graphics() {
                idx = Some(candidate_idx);
                break;
            }
        }

        idx
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

    fn run(&mut self) {
        let mut should_run = true;
        while should_run {
            self.events_loop.poll_events(|ev| {
                log::debug!("Recieved window event {:?}", ev);

                // Should we close?
                if let Event::WindowEvent { event: WindowEvent::CloseRequested, .. } = ev {
                    should_run = false;
                }
            });
        }
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

    let mut app = TutorialApp::new();
    app.run();
}
