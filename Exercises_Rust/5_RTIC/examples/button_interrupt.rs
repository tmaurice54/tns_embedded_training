#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_halt as _;
 // global logger + panicking-behavior + memory layout

#[rtic::app(
    device = stm32f4xx_hal::pac, 
    peripherals = true
)]
mod app {
    //Here goes your imports
    use stm32f4xx_hal::{
        gpio::{self, Edge, Input, Output, PushPull},
        prelude::*,
    };

    // Shared resources go here
    #[shared]
    struct Shared {
        // No ressource shared since we have only one task
    }

    // Local resources go here
    #[local]
    struct Local {
        button: gpio::PC13<Input>,
        led: gpio::PA5<Output<PushPull>>,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut device = cx.device;

        // Configure the led
        let gpioa = device.GPIOA.split();
        let led = gpioa.pa5.into_push_pull_output();

        // Configure the button
        let gpioc = device.GPIOC.split();
        let mut button = gpioc.pc13;

        // Promote SYSCFG structure to HAL to be able to configure interrupts
        let mut syscfg = device.SYSCFG.constrain();

        // Create and allow interrupt on button (Rising edge)
        button.make_interrupt_source(&mut syscfg);
        button.trigger_on_edge(&mut device.EXTI, Edge::Rising);
        button.enable_interrupt(&mut device.EXTI);

        // Promote RCC structure to HAL and configure the clock
        let rcc = device.RCC.constrain();
        let _clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

        (
            // Initialization of shared resources
            Shared { },
            // Initialization of task local resources
            Local {
                button,
                led,
            },
            init::Monotonics(),
        )
    }

    // Background task, runs whenever no other tasks are running
    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            continue;
        }
    }

    #[task(binds = EXTI15_10, local = [led, button])]
    fn button_pressed(cx: button_pressed::Context) {

        // Toggle the led from the global context
        cx.local.led.toggle();

        // Obtain access to Button Peripheral and Clear Interrupt Pending Flag
        cx.local.button.clear_interrupt_pending_bit();
    }
}