use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Polyline, PrimitiveStyle},
};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};

const PADDING: i32 = 8;

fn main() -> Result<(), core::convert::Infallible> {
    let (w, h) = (176i32, 256i32);

    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(w as u32, 170));

    let line_style = PrimitiveStyle::with_stroke(Rgb888::GREEN, 1);

    let points = [
        Point::new(PADDING, h / 2),
        Point::new(50, h / 2),
        Point::new(60, h / 2 - 20),
        Point::new(70, h / 2),
        Point::new(80, h / 2),
        Point::new(90, h / 2 + 10),
        Point::new(100, PADDING),
        Point::new(110, h / 2 + 20),
        Point::new(120, h / 2),
        Point::new(160, h / 2),
    ];

    Polyline::new(&points)
        .into_styled(line_style)
        .draw(&mut display)?;

    Window::new("Polyline", &OutputSettings::default()).show_static(&display);

    Ok(())
}
