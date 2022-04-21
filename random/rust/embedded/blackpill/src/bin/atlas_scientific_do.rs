#![no_std]
#![no_main]
#![deny(unsafe_code)]

use core::fmt::Write;
use core::panic::PanicInfo;

use cortex_m_rt::entry;
use rtt_target::{rprint, rprintln, rtt_init_print};
use stm32f4xx_hal::{
    block,
    pac,
    prelude::*,
    serial::config::Config,
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

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr
        .sysclk(84.MHz())
        .pclk1(42.MHz())
        .pclk2(84.MHz())
        .freeze();

    let gpioa = dp.GPIOA.split();
    let gpioc = dp.GPIOC.split();

    // Init status LED
    let mut led = gpioc.pc13.into_push_pull_output();
    led.set_high();  // off

    // Init UART
    let tx_pin = gpioa.pa2.into_alternate();
    let rx_pin = gpioa.pa3.into_alternate();
    let serial = dp
        .USART2
        .serial(
            (tx_pin, rx_pin),
            Config::default()
                .baudrate(9600.bps()),
            &clocks,
        )
        .unwrap()
        .with_u8_data();

    let (mut tx, mut rx) = serial.split();

    led.set_low();  // on

    loop {
        // Read message as array of ascii chars.
        let mut buf = [b'\0' as char; 64];  // Datasheet doesn't define a max message length afaict :/
        let mut i = 0;
        loop {
            let c = block!(rx.read()).unwrap() as char;

            // End of individual messages are defined by carriage returns.
            if c as u8 == b'\r' { break; }

            buf[i] = c;
            i += 1;
        }

        // Print message
        for c in buf {
            if c as u8 != b'\0' {
                rprint!("{}", c);
            } else {
                rprintln!();
                break;
            }
        }
        //// Parse message
        //loop {
        //    let i = buf
        //        .iter()
        //        .partition_in_place(|&n| n == b',');
        //}

    }
}
