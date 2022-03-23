#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Set up onboard LED pin.
    let gpioc = dp.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output();

    // Set up the system clock.
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

    let mut delay = cp.SYST.delay(&clocks);

    loop {
        rprintln!("Hello world.");
        led.set_high();
        delay.delay_ms(500_u32);
        led.set_low();
        delay.delay_ms(500_u32);
    }
}
