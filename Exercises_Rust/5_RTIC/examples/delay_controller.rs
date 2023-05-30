#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_halt as _;

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true)]
mod app {

    // Import the HAL library
    use stm32f4xx_hal::{
        gpio::{self, Edge, Input, Output, PushPull},
        pac::TIM2,
        prelude::*,
        timer::{self, Event},
    };

    // Resources shared between tasks
    #[shared]
    struct Shared {
        timer: timer::CounterMs<TIM2>,
    }

    // Local resources to specific tasks (cannot be shared)
    #[local]
    struct Local {
        button: gpio::PC13<Input>,
        led: gpio::PA5<Output<PushPull>>,
        global_delay: u32,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {

        // Get device from context
        let mut device = ctx.device;

        // Set let
        let gpioa = device.GPIOA.split();
        let led = gpioa.pa5.into_push_pull_output();

        // Set button
        let gpioc = device.GPIOC.split();
        let mut button = gpioc.pc13;

        // Set the interruption on EXTI
        let mut syscfg = device.SYSCFG.constrain();

        button.make_interrupt_source(&mut syscfg);
        button.trigger_on_edge(&mut device.EXTI, Edge::Rising);
        button.enable_interrupt(&mut device.EXTI);

        // Set the clock
        let rcc = device.RCC.constrain();
        let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

        // Set the timer
        let mut timer = device.TIM2.counter_ms(&clocks);

        // We start te timer with a deley of 2000 ms
        timer.start(2000.millis()).unwrap();

        // Set up interrupt when timer expires
        timer.listen(Event::Update);

        (
            // Initialization of shared resources
            Shared { timer },
            // Initialization of task local resources
            Local {
                button,
                led,
                global_delay: 2000_u32,
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
            // Wait for interrupt
            cortex_m::asm::wfi();
        }
    }

    #[task(binds = EXTI15_10, local = [global_delay, button], shared=[timer])]
    fn button_pressed(mut ctx: button_pressed::Context) {

        // Copy the delay from global context
        let mut delay = *ctx.local.global_delay;

        // Adjust the amount of delay
        delay = delay - 500_u32;
        if delay < 500_u32 {
            delay = 2000_u32;
        }

        // Update delay value in global context
        *ctx.local.global_delay = delay;

        // Update the timeout value in the timer peripheral
        ctx.shared
            .timer
            .lock(|tim| tim.start(delay.millis()).unwrap());

        // Clear Interrupt Pending Flag
        ctx.local.button.clear_interrupt_pending_bit();
    }

    #[task(binds = TIM2, local=[led], shared=[timer])]
    fn timer_expired(mut ctx: timer_expired::Context) {
        
        // Toggle the led
        ctx.local.led.toggle();

        // Clear interrupt on timer
        ctx.shared
            .timer
            .lock(|tim| tim.clear_interrupt(Event::Update));
    }
}