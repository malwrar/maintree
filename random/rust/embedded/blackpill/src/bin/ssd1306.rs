//! Draws stuff on a 128x32 oled screen driven by an ssd1306 chip!

#![no_std]
#![no_main]
#![deny(unsafe_code)]

use core::fmt::Write;

use panic_halt as _;

use cortex_m_rt::entry;

use display_interface_spi::SPIInterface;

use stm32f4xx_hal::{
    pac,
    prelude::*,
    spi::{Spi, Mode, NoMiso, Phase, Polarity},
    timer::Timer,
};

//use ssd1351::{
//    builder::Builder,
//    mode::GraphicsMode,
//    prelude::*,
//    properties::DisplayRotation,
//};

use ssd1306::{
    mode::TerminalMode,
    prelude::*,
    Ssd1306,
};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr
        .sysclk(48.MHz())
        .freeze();

    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();

    // LED
    let mut led = gpioc.pc13.into_push_pull_output();
    led.set_high();  // off

    // SPI3
    let sck     = gpiob.pb3.into_alternate();
    //let miso    = gpiob.pb4.into_alternate();
    let mosi    = gpiob.pb5.into_alternate();
    let mut cs1 = gpiob.pb6.into_push_pull_output();

    let spi3 = Spi::new(
        dp.SPI3,
        (sck, NoMiso {}, mosi),
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        2000u32.kHz(),  // TODO: this is probably wrong
        &clocks,
    );

    let mut delay = Timer::syst(cp.SYST, &clocks).delay();

    // Set up display
    let dc      = gpiob.pb7.into_push_pull_output();
    let mut rst = gpiob.pb8.into_push_pull_output();

    cs1.set_high();  // SPI is active low
    let interface = SPIInterface::new(spi3, dc, cs1);

    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x32,
        DisplayRotation::Rotate0
    ).into_terminal_mode();

    // Reset display
    rst.set_high();
    delay.delay_ms(1_u32);
    rst.set_low();
    delay.delay_ms(10_u32);
    rst.set_high();

    // Initialize display
    display.init().unwrap();
    display.clear().unwrap();

    // Write a basic string in teriminal mode to the display.
    display.write_str("Hello world.");

    //display.set_pixel(1, 1, true);
    //display.flush();

    //display.reset(&mut rst, &mut delay).unwrap();
    //display.init().unwrap();
    //display.set_rotation(DisplayRotation::Rotate0).unwrap();
    //display.clear();

    //for x in 0..256 {
    //    for y in 0..256 {
    //        display.set_pixel(x, y, 0xffff);
    //    }
    //}

    led.set_low();  // indicate that program is donezo.

    loop {
    }
}
