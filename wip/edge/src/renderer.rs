//! 

use std::fmt;

pub trait Surface {

}

/// Object that can be rendered.
pub trait Renderable {

}

/// Thing that can draw a renderable.
pub struct Renderer<R, S>
where
    R: fmt::Debug + Renderable,
    S: Surface,
{
    target: Option<R>,
    surface: S,
}

struct DummySurface {}
impl Surface for DummySurface {}

impl<R, S> Renderer<R, S>
where
    R: fmt::Debug + Renderable,
    S: Surface,
{
    fn new(surface: S) -> Self {
        Self {
            target: None,
            surface,
        }
    }

    fn window() -> Self {
        Self::new(DummySurface {})
    }

    /// Begin drawing this object
    fn draw(&mut self, renderable: R) {
        self.target = Some(renderable);
    }
}