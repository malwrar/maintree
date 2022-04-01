#![no_std]
#![no_main]

use core::{
    cell::RefCell,
    panic::PanicInfo,
};
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{
    gpio::{self, Output, PushPull},
    pac::{self, interrupt, Interrupt, TIM2},
    prelude::*,
    timer::{CounterUs, Event},
};

struct System {
    status_led: gpio::PC13<Output<PushPull>>,
    tim2: CounterUs<TIM2>,
}

static SYSTEM: Mutex<RefCell<Option<System>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn TIM2() {
    rprintln!("Samping...");

    cortex_m::interrupt::free(|cs| {
        let mut system = SYSTEM
            .borrow(cs)          // Mutex
            .borrow_mut();       // RefCell
        let system = system  // "freed while in use"
            .as_mut().unwrap();  // yawn... Option

        system.status_led.toggle();
        system.tim2.wait().unwrap();
    });
}

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
    //let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Init clock as defined in STM32F411xC "on-chip peripheral current consuption" section.
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr
        .sysclk(84.MHz())
        .pclk1(42.MHz())
        .pclk2(84.MHz())
        .freeze();

    let gpioc = dp.GPIOC.split();

    // Init status LED.
    let mut led = gpioc.pc13.into_push_pull_output();
    led.set_high();  // off

    // Init TIM2
    let mut timer = dp.TIM2.counter(&clocks);
    timer.start(500u32.millis()).unwrap();
    timer.listen(Event::Update);

    // Save useful system peripherals to global state.
    cortex_m::interrupt::free(|cs| {
        *SYSTEM.borrow(cs).borrow_mut() = Some(
            System {
                status_led: led,
                tim2: timer,
            }
        )
    });

    // Enable TIM2
    unsafe { cortex_m::peripheral::NVIC::unmask(Interrupt::TIM2); }

    // zzz
    rprintln!("Main core finished.", );
    loop {
        cortex_m::asm::wfi();
    }
}
