#![no_std]
#![no_main]
#![deny(unsafe_code)]

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

/// Really shitty atof implementation, doesn't handle negative numbers or fancy
/// notation.
fn ascii_to_float(float_string: &[u8]) -> f32 {
    let mut characteristic = 0.0f32;
    let mut mantissa       = 0.0f32;
    let mut weight         = 1.0f32;
    let mut in_mantissa    = false;  // float number part terms: characteristic.mantissa

    for c in float_string {
        let c = *c;

        // Skip over any non-float characters
        if (c < b'0' || c > b'9') && c != b'.' {
            continue;
        }

        if c == b'.' {
            in_mantissa = true;
            continue;
        }

        if in_mantissa {
            mantissa *= 10.0;
            mantissa += (c - b'0') as f32;
            weight *= 10.0;
        } else {
            characteristic *= 10.0;
            characteristic += (c - b'0') as f32;
        }
    }

    characteristic + (mantissa / weight)
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Initializing chip...");

    // Init chip
    let _cp = cortex_m::Peripherals::take().unwrap();
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

    let (_tx, mut rx) = serial.split();

    led.set_low();  // on

    loop {
        // Read message as array of ascii chars.
        let mut buf = [b'\0'; 64];  // Datasheet doesn't define a max message length afaict :/
        let mut i = 0;
        loop {
            let c = block!(rx.read()).unwrap();
            if !(c as char).is_ascii() { panic!("Sensor sent non-ascii char."); }

            // End of individual messages are defined by carriage returns.
            if c == b'\r' { break; }

            buf[i] = c;
            i += 1;
        }

        // Print message
        rprint!("got:       ");
        for c in buf {
            if c != b'\0' {
                rprint!("{}", c as char);
            } else {
                rprintln!();
                break;
            }
        }

        for resp in buf.split(|num| *num == b',') {
            if (resp[0] as char).is_ascii_digit() {
                let val = ascii_to_float(resp);
                rprintln!("converted: {}", val);
            }
        }
    }
}
