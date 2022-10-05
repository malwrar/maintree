use std::convert::TryInto;
use std::collections::HashMap;
use std::process::Command;
use std::time::Instant;

use legion::{Entity, World, Resources, Schedule};

use log::{debug, info};

use x11::xlib;

use crate::{
    components::{Position, Orientation},
    systems::draw_window_system,
    error::{
        Error,
        Result,
    },
    render::{
        camera::Camera,
        texture::Texture,
    },
    resources::WindowRenderResource,
    window_system::{
        Display,
        DisplayEvent,
        Window,
        WindowRef,
        NativeWindow,
        NativeDisplay,
        extension::{
            CompositeRedirect,
            CompositeRedirectMode,
            CompositeOverlayWindow,
            DamageWatcher,
            DamageWatcherLevel,
            RenderTarget,
        }
    },
};

/// Window that is managed by a window manager.
#[derive(Debug)]
pub struct ManagedWindow {
    window: WindowRef,
    content: CompositeRedirect,
    damage_watcher: DamageWatcher,
}

impl ManagedWindow {
    pub fn from(window: &WindowRef) -> Result<Self> {
        let content = CompositeRedirect::redirect(window,
                CompositeRedirectMode::Automatic)?;

        Ok(Self {
            window: window.clone(),
            content,
            damage_watcher: DamageWatcher::watch(window,
                    DamageWatcherLevel::RawRectangles)?
        })
    }

    /// Get content as gl texture.
    pub fn get_texture(&self) -> Result<Texture> {
        let attrs = self.window.get_attributes()?;
        self.content.get_content()?
                .to_texture(
                        attrs.width.try_into().unwrap(),
                        attrs.height.try_into().unwrap())
    }
}

impl Window for ManagedWindow {
    fn display(&self) -> Display {
        self.window.display()
    }

    unsafe fn native(&self) -> NativeWindow {
        self.window.native()
    }

    unsafe fn native_display(&self) -> NativeDisplay {
        self.window.native_display()
    }
}

pub struct Compositor {
    display: Display,
    world: World,
    overlay: CompositeOverlayWindow,
    resources: Resources,
    draw_schedule: Schedule,
    managed_windows: HashMap<WindowRef, Entity>,
}

impl Compositor {
    pub fn local() -> Result<Self> {
        let display = Display::local()?;

        // Detect useful events on the Display's root window, which is the
        // common parent for all the top level windows we're interested in
        // managing. Specifically, we'll want to know when...
        let root_window = display.root_window()?;
        root_window.hook_events(
                // ...there are keyboard or mouse button presses.
                xlib::KeyPressMask | xlib::KeyReleaseMask
                | xlib::ButtonPressMask | xlib::ButtonReleaseMask
                // ...the mouse crosses window borders.
                | xlib::EnterWindowMask | xlib::LeaveWindowMask
                // ...the contents of window region(s) have been lost.
                | xlib::ExposureMask
                // ...the input focus changes.
                | xlib::FocusChangeMask
                // ...the window structure has changed (*Notify) or is about
                // to change (*Redirect).
                | xlib::SubstructureRedirectMask | xlib::SubstructureNotifyMask
                // ...a window property has changed.
                | xlib::PropertyChangeMask
        )?;

        // Set up the render window.
        let overlay = CompositeOverlayWindow::get_for(&root_window)?;

        // Set up viewport.
        let attrs = root_window.get_attributes()?;
        let camera = Camera::new(glm::vec3(0.0, 0.0, 1.0), attrs.width,
                attrs.height);

        // Register some of the stuff we've created above as resources in our
        // entity system.
        let mut resources = Resources::default();
        resources.insert(Instant::now());
        resources.insert(display.clone());
        resources.insert(camera);
        resources.insert(WindowRenderResource::new());

        // Create our schedules.
        let draw_schedule = Schedule::builder()
            .add_system(draw_window_system())
            .build();

        Ok(Self {
            display,
            world: World::default(), 
            overlay,
            resources,
            draw_schedule,
            managed_windows: HashMap::new(),
        })
    }

    pub fn manage(&mut self, window: &WindowRef) -> Result<()> {
        info!("Managing {:?}", window);

        // Create an entity for the window.
        let entity = self.world.push(
            (
                Position::origin(),
                Orientation::forward(),
                ManagedWindow::from(window)?,
            )
        );
        self.managed_windows.insert(window.clone(), entity);

        Ok(())
    }

    pub fn unmanage(&mut self, window: &WindowRef) -> Result<()> {
        info!("No longer managing {:?}", window);

        // Destroy the entity for the window.
        match self.managed_windows.remove(window) {
            Some(entity) => Ok(self.world.remove(entity)),
            None => return Err(
                    Error::new("Tried to unmanage an unmanaged window."))
                    //format!("Tried to unmanage an unmanaged window: {:?}",
                    //        window)))
        }?;

        Ok(())
    }

    pub fn is_managed(&self, window: &WindowRef) -> bool {
        self.managed_windows.contains_key(window)
    }

    /// Render the window manager to 
    pub fn render(&mut self) -> Result<()> {
        self.overlay.start_frame()?;

        self.draw_schedule.execute(&mut self.world, &mut self.resources);

        self.overlay.render_frame()?;

        Ok(())
    }

    /// Run a program or command in the system's `$PATH`.
    pub fn exec(&self, command: &str, args: &[&str]) {
        Command::new(command.trim())
            .args(args)
            .spawn()
            .unwrap();
    }

    pub fn run(&mut self) -> Result<()> {
        self.render()?;  // perform an initial render

        loop {
            let event = self.display.consume_next_event_blocking()?;
            match event {
                DisplayEvent::WindowContentChanged(_)
                | DisplayEvent::WindowMapped(_, _) => {
                    self.render()?;
                }
                DisplayEvent::WindowRequestsConfiguration(window, event_mask,
                        changes) => {
                    window.configure(event_mask, changes)?;
                }
                DisplayEvent::WindowRequestsMapping(window, _) => {
                    self.manage(&window)?;
                    window.show()?;
                    self.render()?;
                }
                DisplayEvent::WindowUnmapped(window, _) => {
                    if self.is_managed(&window) {
                        self.unmanage(&window)?;
                    }
                },
                _e => {
                    self.render()?;  // HACK: havent figure out how to catch xdamage events yet!
                    //debug!("Unhandled event: {:?}", e);
                }
            }
        }
    }
}
