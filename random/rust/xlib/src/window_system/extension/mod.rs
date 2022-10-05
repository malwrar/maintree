pub mod damage;
pub mod composite;
pub mod gl;

use crate::error::Result;

pub use self::{
    damage::{
        DamageWatcher,
        DamageWatcherLevel,
    },
    composite::{
        CompositeOverlayWindow,
        CompositeRedirect,
        CompositeRedirectMode,
    },
    gl::GlWindow,
};

pub trait RenderTarget {
    /// Begin setting up a frame.
	fn start_frame(&self) -> Result<()>;

    /// Push the current frame to the render surface.
	fn render_frame(&self) -> Result<()>;
}
