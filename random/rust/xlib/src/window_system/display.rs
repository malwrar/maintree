use std::convert::TryInto;
use std::hash::{Hash, Hasher};
use std::os::raw::{c_char, c_int, c_uint};
use std::ptr;
use std::sync::Arc;
use x11::xlib;
use crate::{
    error::{Result, Error},
    window_system::WindowRef,
};

pub type NativeDisplay = *mut xlib::Display;

/// Manages a connection to an x11 display.
#[derive(Debug)]
pub struct DisplayHandle {
    native: NativeDisplay,
}

impl DisplayHandle {
    pub fn open(name: *const c_char) -> Result<Self> {
        let native = unsafe {
            xlib::XOpenDisplay(name)
        };
        
        if native.is_null() {
            return Err(Error::new("Failed to open display."));
            //return Err(Error::new(format!("Failed to open display: {:?}",
            //        native)));
        }

        Ok(Self {
            native
        })
    }

    /// Get the native display object.
    pub unsafe fn native(&self) -> NativeDisplay {
        self.native
    }
}

impl Drop for DisplayHandle {
    fn drop(&mut self) {
        println!("Dropped display.");
        unsafe {
            xlib::XCloseDisplay(self.native);
        }
    }
}

// I'm fine mom I know what I'm doing
unsafe impl Send for DisplayHandle {}
unsafe impl Sync for DisplayHandle {}

/// Events that can be sent by 
#[derive(Debug)]
pub enum DisplayEvent {
    // TODO: don't use native event types

    KeyPressed(xlib::XKeyPressedEvent),
    KeyReleased(xlib::XKeyReleasedEvent),

    MouseMoved(xlib::XMotionEvent),
    MouseButtonPressed(xlib::XButtonPressedEvent),
    MouseButtonReleased(xlib::XButtonPressedEvent),

    WindowCreated(WindowRef, xlib::XCreateWindowEvent),
    WindowDestroyed(WindowRef, xlib::XDestroyWindowEvent),
    WindowEntered(WindowRef, xlib::XEnterWindowEvent),
    WindowExited(WindowRef, xlib::XEnterWindowEvent),
    WindowContentChanged(WindowRef),
    WindowReparented(WindowRef, xlib::XReparentEvent),
    WindowRequestsMapping(WindowRef, xlib::XMapRequestEvent),
    WindowMapped(WindowRef, xlib::XMapEvent),
    WindowUnmapped(WindowRef, xlib::XUnmapEvent),
    WindowRequestsConfiguration(WindowRef, c_uint, xlib::XWindowChanges),
    WindowConfigured(WindowRef, xlib::XConfigureEvent),
    WindowRequestsStackPositionChange(WindowRef, xlib::XCirculateRequestEvent),
    WindowStackPositionChanged(WindowRef, xlib::XCirculateEvent),
    WindowFocused(WindowRef, xlib::XFocusInEvent),
    WindowUnfocused(WindowRef, xlib::XFocusOutEvent),
    WindowMovedWithParent(WindowRef, xlib::XGravityEvent),

    Unknown(xlib::XEvent),
}

/// Interface for interacting with an x11 display.
#[derive(Debug, Clone)]
pub struct Display {
    handle: Arc<DisplayHandle>,
}

impl Display {
    /// Open a connection to a x11 display.
    pub fn local() -> Result<Self> {
        let handle = DisplayHandle::open(ptr::null_mut())?;

        Ok(Self {
            handle: Arc::new(handle),
        })
    }

    pub fn root_window(&self) -> Result<WindowRef> {
        let window_id = unsafe {
            xlib::XDefaultRootWindow(self.handle.native())
        };

        Ok(WindowRef::from_native(&self, window_id))
    }

    pub fn default_screen(&self) -> Result<c_int> {
        let screen_id = unsafe {
            xlib::XDefaultScreen(self.handle.native())
        };

        Ok(screen_id)
    }

    /// Turn on/off synchronization
    pub fn sync(&self, on: bool) -> Result<()> {
        let on = match on {
            true => xlib::True,
            false => xlib::False,
        };

        unsafe {
            xlib::XSynchronize(self.handle.native(), on);
        }

        Ok(())
    }

    /// Consume the next event for this display, blocking if there's none available.
    pub fn consume_next_event_blocking(&mut self) -> Result<DisplayEvent> {
        // TODO: make an iter version of this? 

        // Block on next native event
        let mut xevent = xlib::XEvent { pad: [0;24] };
        unsafe {
            xlib::XNextEvent(self.handle.native(), &mut xevent);
        };

        // Translate native event into something more usable.
        let event = match xevent.get_type() {
            xlib::KeyPress => {
                let xevent = xlib::XKeyPressedEvent::from(xevent);
                DisplayEvent::KeyPressed(xevent)
            },
            xlib::KeyRelease => {
                let xevent = xlib::XKeyReleasedEvent::from(xevent);
                DisplayEvent::KeyReleased(xevent)
            },
            xlib::MotionNotify => {
                let xevent = xlib::XMotionEvent::from(xevent);
                DisplayEvent::MouseMoved(xevent)
            },
            xlib::ButtonPress => {
                let xevent = xlib::XButtonPressedEvent::from(xevent);
                DisplayEvent::MouseButtonPressed(xevent)
            },
            xlib::ButtonRelease => {
                let xevent = xlib::XButtonReleasedEvent::from(xevent);
                DisplayEvent::MouseButtonReleased(xevent)
            },
            xlib::CreateNotify => {
                let xevent = xlib::XCreateWindowEvent::from(xevent);
                let window = WindowRef::from_native(&self, xevent.window);
                DisplayEvent::WindowCreated(window, xevent)
            },
            xlib::DestroyNotify => {
                let xevent = xlib::XDestroyWindowEvent::from(xevent);
                let window = WindowRef::from_native(&self, xevent.window);
                DisplayEvent::WindowDestroyed(window, xevent)
            },
            xlib::EnterNotify => {
                let xevent = xlib::XEnterWindowEvent::from(xevent);
                let window = WindowRef::from_native(&self, xevent.window);
                DisplayEvent::WindowEntered(window, xevent)
            },
            xlib::LeaveNotify => {
                let xevent = xlib::XLeaveWindowEvent::from(xevent);
                let window = WindowRef::from_native(&self, xevent.window);
                DisplayEvent::WindowExited(window, xevent)
            },
            xlib::Expose => {
                // TODO: get compositor WindowContentHandle
                let xevent = xlib::XExposeEvent::from(xevent);
                let window = WindowRef::from_native(&self, xevent.window);
                DisplayEvent::WindowContentChanged(window)
            },
            xlib::ReparentNotify => {
                let xevent = xlib::XReparentEvent::from(xevent);
                let window = WindowRef::from_native(&self, xevent.window);
                DisplayEvent::WindowReparented(window, xevent)
            },
            // Sent when a client calls XMapWindow() to make the window
            // visible on screen.
            xlib::MapRequest => {
                let xevent = xlib::XMapRequestEvent::from(xevent);
                let window = WindowRef::from_native(&self, xevent.window);
                DisplayEvent::WindowRequestsMapping(window, xevent)
            },
            xlib::MapNotify => {
                let xevent = xlib::XMapEvent::from(xevent);
                let window = WindowRef::from_native(&self, xevent.window);
                DisplayEvent::WindowMapped(window, xevent)
            },
            xlib::UnmapNotify => {
                let xevent = xlib::XUnmapEvent::from(xevent);
                let window = WindowRef::from_native(&self, xevent.window);
                DisplayEvent::WindowUnmapped(window, xevent)
            },
            // Sent when a client calls XConfigureWindow() to configure the
            // initial position, size, etc of a window.
            xlib::ConfigureRequest => {
                let xevent = xlib::XConfigureRequestEvent::from(xevent);
                let changes = xlib::XWindowChanges {
                    x: xevent.x,
                    y: xevent.y,
                    width: xevent.width,
                    height: xevent.height,
                    border_width: xevent.border_width,
                    sibling: xevent.above,
                    stack_mode: xevent.detail,
                };
                let window = WindowRef::from_native(&self, xevent.window);
                DisplayEvent::WindowRequestsConfiguration(window,
                        xevent.value_mask.try_into().unwrap(), changes)
            },
            xlib::ConfigureNotify => {
                let xevent = xlib::XConfigureEvent::from(xevent);
                let window = WindowRef::from_native(&self, xevent.window);
                DisplayEvent::WindowConfigured(window, xevent)
            },
            // https://tronche.com/gui/x/xlib/events/window-state-change/circulate.html
            xlib::CirculateRequest => {
                let xevent = xlib::XCirculateRequestEvent::from(xevent);
                let window = WindowRef::from_native(&self, xevent.window);
                DisplayEvent::WindowRequestsStackPositionChange(window, xevent)
            },
            xlib::CirculateNotify => {
                let xevent = xlib::XCirculateEvent::from(xevent);
                let window = WindowRef::from_native(&self, xevent.window);
                DisplayEvent::WindowStackPositionChanged(window, xevent)
            },
            // https://tronche.com/gui/x/xlib/events/input-focus/normal-and-grabbed.html
            xlib::FocusIn => {
                let xevent = xlib::XFocusInEvent::from(xevent);
                let window = WindowRef::from_native(&self, xevent.window);
                DisplayEvent::WindowFocused(window, xevent)
            },
            // https://tronche.com/gui/x/xlib/events/input-focus/normal-and-grabbed.html
            xlib::FocusOut => {
                let xevent = xlib::XFocusOutEvent::from(xevent);
                let window = WindowRef::from_native(&self, xevent.window);
                DisplayEvent::WindowUnfocused(window, xevent)
            },
            // https://tronche.com/gui/x/xlib/events/window-state-change/gravity.html
            xlib::GravityNotify => {
                let xevent = xlib::XGravityEvent::from(xevent);
                let window = WindowRef::from_native(&self, xevent.window);
                DisplayEvent::WindowMovedWithParent(window, xevent)
            },
            _ => {
                DisplayEvent::Unknown(xevent)
            }
        };

        Ok(event)
    }

    /// Get the native display object from our handle.
    pub unsafe fn native(&self) -> NativeDisplay {
        self.handle.native()
    }

}

impl PartialEq for Display {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            self.handle.native() == other.handle.native()
        }
    }
}

impl Eq for Display {}

impl Hash for Display {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.handle.native().hash(state);
        }
    }
}

impl std::fmt::Display for Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Display")
                .field("native", &self.handle.native)
                .finish()
    }
}
