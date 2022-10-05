use std::hash::{Hash, Hasher};
use std::mem;
use std::os::raw::{c_long, c_ulong, c_int, c_uint};
use x11::xlib;
use crate::{
    error::Result,
    window_system::{
        Display,
        NativeDisplay,
    },
};

pub type NativeWindow = xlib::Window;

pub trait Window {
    /// Returns the display this window belongs to.
    fn display(&self) -> Display;

    /// Returns the native identifier of this window.
    unsafe fn native(&self) -> NativeWindow;

    /// Returns the native display identifier associated with this window.
    unsafe fn native_display(&self) -> NativeDisplay;

    /// Destroy the window.
    fn destroy(&self) -> Result<()> {
        // TODO: make this consume self?
        unsafe {
            xlib::XDestroyWindow(self.native_display(), self.native());
        };
        // TODO: check for errors
        
        Ok(())
    }

    /// Configure window to send events to its display.
    fn hook_events(
        &self,
        xlib_eventmask: c_long
    ) -> Result<()> {
        //self.display_server.synchronize(true);

        unsafe {
            xlib::XSelectInput(self.native_display(),
                    self.native(), xlib_eventmask);
        }
        // TODO: check for errors

        //self.display_server.synchronize(false);

        Ok(())
    }

    /// Clear any event hooks we might have set on this window.
    fn clear_hooks(&mut self) -> Result<()> {
        Ok(self.hook_events(xlib::NoEventMask)?)
    }

    /// Determines if this window is resident in a display.
    fn is_resident(&self, display: &Display) -> bool {
        self.display() == *display
    }

    /// Determines if this window is on the same display as another window.
    fn is_coresident(&self, window: &Self) -> bool {
        self.is_resident(&window.display()) 
    }

    /// Configure the window.
    fn configure(
        &self,
        value_mask: c_uint,
        mut changes: xlib::XWindowChanges,
    ) -> Result<()> {
        unsafe {
            xlib::XConfigureWindow(self.native_display(), self.native(),
                    value_mask, &mut changes);
        }
        // TODO: check for errors
        
        Ok(())
    }

    /// Display the window.
    fn show(&self) -> Result<()> {
        unsafe {
            xlib::XMapWindow(self.native_display(), self.native());
        }
        // TODO: check for errors
        
        Ok(())
    }

    /// Hide the window.
    fn hide(&self) -> Result<()> {
        unsafe {
            xlib::XUnmapWindow(self.native_display(), self.native());
        }
        // TODO: check for errors
        
        Ok(())
    }

    /// Get properties of a window.
    fn get_attributes(&self) -> Result<xlib::XWindowAttributes> {
        let attrs = unsafe {
            let mut attrs: xlib::XWindowAttributes = mem::zeroed();
            xlib::XGetWindowAttributes(self.native_display(), self.native(),
                    &mut attrs);

            attrs
        };
        // TODO: check for errors
        
        Ok(attrs)
    }

    ///
    fn set_size(&self, width: u32, height: u32) -> Result<()> {
        unsafe {
            xlib::XResizeWindow(self.native_display(), self.native(), width,
                    height);
        }
        // TODO: check for errors

        Ok(())
    }

    ///
    fn set_pos(&self, x: i32, y: i32) -> Result<()> {
        unsafe {
            xlib::XMoveWindow(self.native_display(), self.native(), x, y);
        }
        // TODO: check for errors

        Ok(())
    }

    /// Establish a parent-child relationship between two windows.
    fn set_parent(&self, parent: &Self) -> Result<()> {
        // TODO: get pos of parent
        let x = 0;
        let y = 0;

        unsafe {
            xlib::XReparentWindow(self.native_display(), self.native(),
                    parent.native(), x, y);
        }
        // TODO: check for errors

        Ok(())
    }

    /// Ask the display server not to kill a window after our client exits.
    ///
    /// When our client exits, all of the resources it created
    /// [will be destroyed][1] by the display server. This means that if we've
    /// reparented any windows we don't own as children of windows we do own,
    /// those children will be destroyed alongside the parents.
    ///
    /// This is mostly relevant for window managers who frame client windows,
    /// as the framing window will kill the child it frames unless we
    /// [explicitly preserve it][2].
    ///
    /// [1]: https://tronche.com/gui/x/xlib/display/close-operation.html
    /// [2]: https://tronche.com/gui/x/xlib/window-and-session-manager/controlling-window-lifetime.html:
    fn preserve(&self) -> Result<()> {
        unsafe {
            xlib::XAddToSaveSet(self.native_display(), self.native());
        }
        // TODO: check for errors

        Ok(())
    }

    /// Inverse of `preserve_window_after_exit`.
    fn unpreserve(&self) -> Result<()> {
        unsafe {
            xlib::XRemoveFromSaveSet(self.native_display(), self.native());
        }
        // TODO: check for errors

        Ok(())
    }
}

/// Simple reference to a window on some display.
#[derive(Clone, Debug)]
pub struct WindowRef {
    native: NativeWindow,
    display: Display,
}

impl WindowRef {
    /// Derive the Window from a native reference to one.
    pub fn from_native(
        display: &Display,
        native: NativeWindow
    ) -> Self {
        Self {
            native,
            display: display.clone(),
        }
    }

    /// Create a Window from scratch.
    /// 
    /// Pretty much a wraper over [XCreateWindow][1]
    ///
    /// 1: https://tronche.com/gui/x/xlib/window/XCreateWindow.html
    pub fn create(
        parent: &WindowRef,
        x: c_int,
        y: c_int,
        width: c_uint,
        height: c_uint,
        border_width: c_uint,
        depth: c_int,
        class: c_uint,
        visual: *mut xlib::Visual,
        valuemask: c_ulong,
        attributes: *mut xlib::XSetWindowAttributes,
    ) -> Result<Self> {
        // Create a new reference to the display used by the parent, ensuring
        // that the child isn't referencing a different display.
        let window = unsafe {
            xlib::XCreateWindow(parent.native_display(), parent.native(), x, y, width,
                    height, border_width, depth, class, visual, valuemask, attributes)
        };
        // TODO: check for errors

        Ok(Self::from_native(&parent.display(), window))
    }
}

impl Window for WindowRef {
    fn display(&self) -> Display {
        self.display.clone()
    }

    unsafe fn native(&self) -> NativeWindow {
        self.native
    }

    unsafe fn native_display(&self) -> NativeDisplay {
        self.display.native()
    }
}

impl PartialEq for WindowRef {
    fn eq(&self, other: &Self) -> bool {
        self.native == other.native
    }
}

impl Eq for WindowRef {}

impl Hash for WindowRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.native.hash(state);
    }
}
