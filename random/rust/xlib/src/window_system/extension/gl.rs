use std::convert::TryInto;
use std::ffi::CString;
use std::mem;
use std::os::raw::{c_int, c_void};
use std::ptr;
use std::sync::Arc;
use gl;
use x11::glx;
use x11::xlib;
use crate::{
    error::{
        Error,
        Result
    },
    window_system::{
        Display,
        Window,
        WindowRef,
        NativeWindow,
        NativeDisplay,
        extension::{
            RenderTarget,
        },
    },
};

const GLX_CONTEXT_MAJOR_VERSION_ARB: u32 = 0x2091;
const GLX_CONTEXT_MINOR_VERSION_ARB: u32 = 0x2092;

const FB_ATTRS: [c_int; 15] = [
    glx::GLX_DOUBLEBUFFER, xlib::True,
    glx::GLX_RED_SIZE, 8,
    glx::GLX_GREEN_SIZE, 8,
    glx::GLX_BLUE_SIZE, 8,
    glx::GLX_DEPTH_SIZE, 8,
    glx::GLX_STENCIL_SIZE, 8,
    glx::GLX_ALPHA_SIZE, 8,
    0
];

const VISUAL_ATTRS: [c_int; 15] = [
    glx::GLX_RGBA, glx::GLX_DOUBLEBUFFER,
    glx::GLX_RED_SIZE, 8,
    glx::GLX_GREEN_SIZE, 8,
    glx::GLX_BLUE_SIZE, 8,
    glx::GLX_DEPTH_SIZE, 8,
    glx::GLX_STENCIL_SIZE, 8,
    glx::GLX_ALPHA_SIZE, 8,
    0
];

type GlXCreateContextAttribsARBProc = unsafe extern "C" fn(
    display: *mut xlib::Display,
    fb_config: glx::GLXFBConfig,
    share_context: glx::GLXContext,
    direct: xlib::Bool,
    attrs: *const c_int,
) -> glx::GLXContext;

/// Safe wrapper over an x11 glcontext.
#[derive(Debug)]
struct GlContext {
    display: Display,
    ctx: glx::GLXContext
}

impl GlContext {
    pub fn new(
        display: &Display,
        gl_major: c_int,
        gl_minor: c_int,
    ) -> Result<Self> {
        check_gl_extension(display)?;

        // Ensure that gl is using the extension's get_proc_address.
        gl::load_with(|s| {
            get_proc_address(s).unwrap_or_else(|e| {
                panic!("Failed to find gl func {}: {:?}", s, e);
            })
        });

        unsafe { gl::Enable(gl::DEPTH_TEST); }

        let create_context_attribs: GlXCreateContextAttribsARBProc = unsafe {
            let addr = get_proc_address("glXCreateContextAttribsARB")
                .unwrap_or_else(|e| {
                    panic!("Failed to find glXCreateContextAttribsARB: {:?}", e);
                });
            mem::transmute(addr)
        };

        let context_attribs = [
            GLX_CONTEXT_MAJOR_VERSION_ARB as c_int, gl_major,
            GLX_CONTEXT_MINOR_VERSION_ARB as c_int, gl_minor,
            0,
        ];

        // Select renderable fb config
        let fb_config = choose_fb_config(display, &FB_ATTRS)?;

        let ctx = unsafe {
            create_context_attribs(display.native(), fb_config,
                    ptr::null_mut(), xlib::True, context_attribs.as_ptr())
        };
        if ctx.is_null() {
            return Err(Error::new("Error when creating context"));
        }

        if unsafe { glx::glXIsDirect(display.native(), ctx) } == 0 {
            return Err(Error::new("Obtained indirect rendering context"));
        }

        Ok(Self {
            display: display.clone(),
            ctx
        })
    }

    fn activate(&self, window: &WindowRef) -> Result<()> {
        if !window.is_resident(&self.display) {
            return Err(Error::new("Can't activate GLContext: target Window on \
                     different display."));
        }

        unsafe {
            glx::glXMakeCurrent(self.display.native(), window.native(),
                    self.ctx);
        }

        Ok(())
    }

    fn deactivate(&self) {
        unsafe {
            glx::glXMakeCurrent(self.display.native(), 0, ptr::null_mut());
        }
    }
}

impl Drop for GlContext {
    fn drop(&mut self) {
        self.deactivate();

        unsafe {
            glx::glXDestroyContext(self.display.native(), self.ctx);
        }
    }
}

// I'm fine mom I know what I'm doing
unsafe impl Send for GlContext {}
unsafe impl Sync for GlContext {}

#[derive(Clone, Debug)]
pub struct GlWindow {
    gl_ctx: Arc<GlContext>,
    render_surface: WindowRef,
}

impl GlWindow {
    pub fn create(
        parent: &WindowRef,
        gl_major: c_int,
        gl_minor: c_int
    ) -> Result<Self> {
        let display = parent.display();

        let visual = choose_visual(&display, &VISUAL_ATTRS)?;

        // Create renderer window on top of parent window.
        let mut attrs: xlib::XSetWindowAttributes = unsafe {
            mem::MaybeUninit::zeroed().assume_init()
        };
        attrs.colormap = unsafe {
            xlib::XCreateColormap(parent.native_display(), parent.native(),
                    visual.visual, xlib::AllocNone)
        };
        attrs.event_mask = xlib::ExposureMask | xlib::KeyPressMask;

        let parent_attrs = parent.get_attributes()?;
        let render_surface = WindowRef::create(&parent,
                parent_attrs.x, parent_attrs.y,
                parent_attrs.width.try_into().unwrap(),
                parent_attrs.height.try_into().unwrap(),
                0, visual.depth, xlib::InputOutput.try_into().unwrap(),
                visual.visual, xlib::CWColormap | xlib::CWEventMask,
                &mut attrs)?;

        render_surface.show()?;

        let gl_ctx = GlContext::new(&display, gl_major, gl_minor)?;
        gl_ctx.activate(&render_surface)?;

        Ok(Self {
            gl_ctx: Arc::new(gl_ctx),
            render_surface,
        })
    }
}

impl Window for GlWindow {
    fn display(&self) -> Display {
        self.render_surface.display()
    }

    unsafe fn native(&self) -> NativeWindow {
        self.render_surface.native()
    }

    unsafe fn native_display(&self) -> NativeDisplay {
        self.render_surface.native_display()
    }
}

impl RenderTarget for GlWindow {
    fn start_frame(&self) -> Result<()> {
        self.gl_ctx.activate(&self.render_surface)?;

        let attrs = self.render_surface.get_attributes()?;
        unsafe {
            gl::Viewport(0, 0, attrs.width, attrs.height);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        };
        // TODO: check errors

        Ok(())
    }

    fn render_frame(&self) -> Result<()> {
        unsafe {
            glx::glXSwapBuffers(self.render_surface.native_display(),
                    self.render_surface.native());
        }
        // TODO: check errors

        self.gl_ctx.deactivate();

        Ok(())
    }
}

/// Contains information about the glx extension.
pub struct GlExtensionInfo {
    pub version_major: c_int,
    pub version_minor: c_int,
    pub event_base: c_int,
    pub error_base: c_int,
}

impl GlExtensionInfo {
    pub fn query(display: &Display) -> Option<Self> {
        let mut info = Self {
            version_major: 0,
            version_minor: 0,
            event_base: 0,
            error_base: 0,
        };

        // Verify the extension is around before continuing
        let status = unsafe {
            glx::glXQueryExtension(display.native(),
                    &mut info.event_base, &mut info.error_base)
        };
        // TODO: check for error

        if status == xlib::False {
            return None;
        }

        unsafe {
            glx::glXQueryVersion(display.native(), &mut info.version_major,
                    &mut info.version_minor);
        }
        // TODO: check for error
        
        Some(info)
    }
}

/// Verify glx is available.
pub fn check_gl_extension(
    display: &Display
) -> Result<GlExtensionInfo> {
    match GlExtensionInfo::query(display) {
        Some(info) => Ok(info),
        None => Err(Error::new("glx not installed on display."))
    }
}

/// 
pub fn get_proc_address(
    name: &str
) -> Result<*mut c_void> {
    let c_name = CString::new(name).unwrap();

    let address: *mut c_void = unsafe {
        mem::transmute(glx::glXGetProcAddress(mem::transmute(c_name.as_ptr())))
    };

    if address.is_null() {
        //return Err(Error::new(format!("Failed to locate gl func \"{:?}\".", c_name)));
        return Err(Error::new("Failed to locate gl func."));
    }

    Ok(address)
}

///
fn choose_visual(
    display: &Display,
    visual_attrs: &[c_int],
) -> Result<xlib::XVisualInfo> {
    check_gl_extension(display)?;

    // Choose a visual for our
    //display.sync(true)?;
    let visual = unsafe {
        glx::glXChooseVisual(display.native(), display.default_screen()?,
                visual_attrs.as_ptr() as *mut c_int)  // <-- this is probably blasphemy
    };
    //display.sync(false)?;
    // TODO: check for error
 
    if visual.is_null() {
        return Err(Error::new("Failed to choose renderable visual."));
    }

    Ok(unsafe { *visual })
}

///
fn choose_fb_config(
    display: &Display,
    fb_attrs: &[c_int],
) -> Result<glx::GLXFBConfig> {
    check_gl_extension(display)?;

    unsafe {
        let mut fb_count: c_int = 0;

        //display.sync(true)?;
        let fb_configs = glx::glXChooseFBConfig(display.native(),
                display.default_screen()?, fb_attrs.as_ptr(), &mut fb_count);
        //display.sync(false)?;

        if fb_count == 0 {
            return Err(Error::new("Failed to find compatible fb config."));
        }

        // Make a copy for us to use of the first fb_config we get and free the
        // rest.
        let fb_config = *fb_configs;
        xlib::XFree(fb_configs as *mut c_void);

        Ok(fb_config)
    }
}
