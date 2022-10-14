use std::os::raw::c_int;
use x11::xlib;
use x11::xcomposite;

pub enum CompositeRedirectMode {
    Automatic,
    Manual
}

/// Window that overlays other windows, providing an opengl RenderTarget.
#[derive(Debug)]
pub struct CompositeOverlayWindow {
	display: xlib::Display,
	overlay_window: xlib::Window,
}

impl CompositeOverlayWindow {
    pub fn over_root(display: xlib::Display) -> Self {
        let (display, overlay_window) = unsafe {
            xlib::XOpenDisplay
        }
    }
    pub fn get_for(display: xlib::Display, window: xlib::Window) -> Result<Self> {
        check_composite_extension(&display)?;

        let (display, overlay_window_id) = unsafe {
            xlib::Display
        let display = target.display();
            xcomposite::XCompositeGetOverlayWindow(target.native_display(),
                    target.native())
        };
        // TODO: check errors

        Ok(Self {
            display,
            target_window_id: window,
            overlay_window_id,
        })
    }
}

impl Window for CompositeOverlayWindow {
    fn display(&self) -> Display {
        self.overlay.display()
    }

    unsafe fn native(&self) -> NativeWindow {
        self.overlay.native()
    }

    unsafe fn native_display(&self) -> NativeDisplay {
        self.overlay.native_display()
    }
}

impl RenderTarget for CompositeOverlayWindow {
    fn start_frame(&self) -> Result<()> {
        self.renderer.start_frame()
    }

    fn render_frame(&self) -> Result<()> {
        self.renderer.render_frame()
    }
}

impl Drop for CompositeOverlayWindow {
    fn drop(&mut self) {
        unsafe {
            xcomposite::XCompositeReleaseOverlayWindow(
                    self.overlay.native_display(), self.overlay.native());
                    
        }
        // TODO: check errors
    }
}

/// Context for redirecting the contents of a window to an off-screen buffer.
#[derive(Debug)]
pub struct CompositeRedirect {
    target: WindowRef,
    mode: c_int,
}

impl CompositeRedirect {
	pub fn redirect(
        target: &WindowRef,
        mode: CompositeRedirectMode,
    ) -> Result<Self> {
        check_composite_extension(&target.display())?;

        // Map mode to native type
        let mode = match mode {
            CompositeRedirectMode::Automatic => xcomposite::CompositeRedirectAutomatic,
            CompositeRedirectMode::Manual => xcomposite::CompositeRedirectManual,
        };

        // TODO: should we give the option to redirect subwindows?
        unsafe {
            xcomposite::XCompositeRedirectWindow(target.native_display(),
                    target.native(), mode);
        }
        // TODO: check errors

        Ok(Self {
            target: target.clone(),
            mode,
        })
    }

	pub fn get_content(&self) -> Result<Pixmap> {
        let pixmap = unsafe {
            xcomposite::XCompositeNameWindowPixmap(
                    self.target.native_display(), self.target.native())
        };
        // TODO: check errors

        Ok(Pixmap::from_native(&self.target.display(), pixmap))
    }
}

impl Drop for CompositeRedirect {
    fn drop(&mut self) {
        unsafe {
            xcomposite::XCompositeUnredirectWindow(self.target.native_display(),
                    self.target.native(), self.mode);
                    
        }
        // TODO: check errors
    }
}

/// Contains information about the Xcomposite extension.
pub struct CompositeExtensionInfo {
    pub version_major: c_int,
    pub version_minor: c_int,
    pub event_base: c_int,
    pub error_base: c_int,
}

impl CompositeExtensionInfo {
    pub fn query(display: &Display) -> Option<Self> {
        let mut info = Self {
            version_major: 0,
            version_minor: 0,
            event_base: 0,
            error_base: 0,
        };

        // Verify the extension is around before continuing
        let status = unsafe {
            xcomposite::XCompositeQueryExtension(display.native(),
                    &mut info.event_base, &mut info.error_base)
        };
        // TODO: check for error

        if status == xlib::False {
            return None;
        }

        unsafe {
            xcomposite::XCompositeQueryVersion(display.native(),
                    &mut info.version_major, &mut info.version_minor);
        }
        // TODO: check for error
        
        Some(info)
    }
}

/// Verify Xcomposite is available.
pub fn check_composite_extension(
    display: &Display
) -> Result<CompositeExtensionInfo> {
    match CompositeExtensionInfo::query(display) {
        Some(info) => Ok(info),
        None => Err(Error::new("Xcomposite not installed on display."))
                        //format!("Xcomposite not installed on display {:?}.",
                        //        display)))
    }
}