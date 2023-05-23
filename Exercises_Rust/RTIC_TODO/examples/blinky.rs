#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_halt as _;

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true, dispatchers=[SPI1])]
mod app {

    use stm32f4xx_hal::{
        gpio::{self, Output, PushPull},
        prelude::*,
    };

    // Resources shared between tasks
    #[shared]
    struct Shared {
        led: gpio::PA5<Output<PushPull>>,
    }

    // Local resources to specific tasks (cannot be shared)
    #[local]
    struct Local {

    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut dp = ctx.device;

        // Configure the LED pin as a push pull ouput and obtain handle
        // On the Nucleo FR401 theres an on-board LED connected to pin PA5
        // 1) Promote the GPIOA PAC struct
        let gpioa = dp.GPIOA.split();
        // 2) Configure Pin and Obtain Handle
        let led = gpioa.pa5.into_push_pull_output();


        // Configure and obtain handle for delay abstraction
        // 1) Promote RCC structure to HAL to be able to configure clocks
        let rcc = dp.RCC.constrain();
        // 2) Configure the system clocks
        // 8 MHz must be used for HSE on the Nucleo-F401RE board according to manual
        let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

        (
            // Initialization of shared resources
            Shared { led },
            // Initialization of task local resources
            Local {},
            // Move the monotonic timer to the RTIC run-time, this enables
            // scheduling
            init::Monotonics(),
        );

        turn_on::spawn().unwrap();
        turn_off::spawn().unwrap();
    }

    // Background task, runs whenever no other tasks are running
    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            // Go to sleep
            cortex_m::asm::wfi();
        }
    }

    #[task(priority = 1)]
    fn turn_on(mut ctx: turn_on::Context) {
        let mut led = *ctx.shared.led;

        if led.is_high() {
            led.set_low();
        }
    }

    #[task(priority = 1)]
    fn turn_off(mut ctx: turn_off::Context) {
        let mut led = *ctx.shared.led;

        if led.is_low() {
            led.set_high();
        }
    }
}