#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_halt as _;

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true)]
mod app {

    use stm32f4xx_hal::{
        gpio::{self, Output, PushPull},
        pac::TIM2,
        prelude::*,
        timer::{self, Event},
    };

    // Resources shared between tasks
    #[shared]
    struct Shared {}

    // Local resources to specific tasks (cannot be shared)
    #[local]
    struct Local {
        timer: timer::CounterMs<TIM2>,
        led: gpio::PA5<Output<PushPull>>
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {

        // get the device from context
        let device = ctx.device;

        // Configure led
        let gpioa = device.GPIOA.split();
        let led = gpioa.pa5.into_push_pull_output();

        // Configure and obtain handle for delay abstraction
        // 1) Promote RCC structure to HAL to be able to configure clocks
        let rcc = device.RCC.constrain();
        // 2) Configure the system clocks
        // 8 MHz must be used for HSE on the Nucleo-F401RE board according to manual
        let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();
        // 3) Create delay handle
        //let mut delay = device.TIM1.delay_ms(&clocks);
        let mut timer = device.TIM2.counter_ms(&clocks);

        // Kick off the timer with 2 seconds timeout first
        // It probably would be better to use the global variable here but I did not to avoid the clutter of having to create a crtical section
        timer.start(200.millis()).unwrap();

        // Set up to generate interrupt when timer expires
        timer.listen(Event::Update);

        (
            // Initialization of shared resources
            Shared { },
            // Initialization of task local resources
            Local {
                timer,
                led
            },
            // Move the monotonic timer to the RTIC run-time, this enables
            // scheduling
            init::Monotonics(),
        )
    }

    // Background task, runs whenever no other tasks are running
    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            // Go to sleep
            cortex_m::asm::wfi();
        }
    }

    #[task(binds = TIM2, local=[led,timer])]
    fn timer_expired(ctx: timer_expired::Context) {
        // When Timer Interrupt Happens Two Things Need to be Done
        // 1) Toggle the LED
        // 2) Clear Timer Pending Interrupt

        ctx.local.led.toggle();
        ctx.local.timer.clear_interrupt(Event::Update);
    }
}