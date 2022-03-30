#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};

use stm32f4xx_hal::{
    pac,
    prelude::*,
    spi::Spi,
    timer::Timer,
};

use ssd1351::{
    builder::Builder,
    mode::{GraphicsMode},
    prelude::*,
};

use embedded_graphics::{
    mono_font::{ascii::FONT_5X8, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{
        Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle,
        StrokeAlignment, Triangle,
    },
    text::{Alignment, Text},
};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("PANIC HANDLER");
    rprintln!("{}", info);

    loop {}
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Initializing chip...");

    // Init chip
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Init clock as defined in STM32F411xC "on-chip peripheral current consuption" section.
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr
        .sysclk(84.MHz())
        .pclk1(42.MHz())
        .pclk2(84.MHz())
        .freeze();

    let mut delay = Timer::syst(cp.SYST, &clocks).delay();

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();

    // Init status LED
    let mut led = gpioc.pc13.into_push_pull_output();
    led.set_high();  // off

    rprintln!("Initializing SPI...");

    let sck2  = gpiob.pb13.into_alternate();
    let miso2 = gpiob.pb14.into_alternate();
    let mosi2 = gpiob.pb15.into_alternate();

    let spi2 = Spi::new(
        dp.SPI2,
        (sck2, miso2, mosi2),
        SSD1351_SPI_MODE,
        100u32.kHz(),
        &clocks
    );

    let dc      = gpioa.pa8.into_push_pull_output();
    let mut rst = gpioa.pa9.into_push_pull_output();
    let mut cs1 = gpioa.pa11.into_push_pull_output();

    let mut display: GraphicsMode<_> = Builder::new()
        .connect_spi(spi2, dc)
        .into();

    cs1.set_low();

    display.reset(&mut rst, &mut delay).unwrap();
    display.init().unwrap();

    let thin_stroke = PrimitiveStyle::with_stroke(Rgb565::GREEN, 1);
    let thick_stroke = PrimitiveStyle::with_stroke(Rgb565::new(200, 0, 200), 3);
    let border_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::RED)
        .stroke_width(3)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();
    let fill = PrimitiveStyle::with_fill(Rgb565::BLUE);
    let character_style = MonoTextStyle::new(&FONT_5X8, Rgb565::WHITE);


    let yoffset = 10;

    display
        .bounding_box()
        .into_styled(border_stroke)
        .draw(&mut display)
        .unwrap();

    Triangle::new(
        Point::new(16 + 8 , yoffset),       // ^
        Point::new(16,      16 + yoffset),  // /
        Point::new(16 + 16, 16 + yoffset))  // \
    .into_styled(thin_stroke)
    .draw(&mut display)
    .unwrap();

    Rectangle::new(Point::new(52, yoffset), Size::new(16, 16))
        .into_styled(fill)
        .draw(&mut display)
        .unwrap();

    Circle::new(Point::new(88, yoffset), 17)
        .into_styled(thick_stroke)
        .draw(&mut display)
        .unwrap();

    Text::with_alignment(
        "Hello world.",
        display.bounding_box().center() + Point::new(0, 15),
        character_style,
        Alignment::Center,
    )
    .draw(&mut display)
    .unwrap();

    cs1.set_high();

    rprintln!("Setup complete!");
    led.set_low();  // on

    loop { }
}
