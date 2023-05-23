#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use panic_halt as _;
 // global logger + panicking-behavior + memory layout

// TODO(7) Configure the `rtic::app` macro
#[rtic::app(
    // TODO: Replace `some_hal::pac` with the path to the PAC
    device = some_hal::pac,
    peripherals = true
)]
mod app {
    //Here goes your imports

    // Shared resources go here
    #[shared]
    struct Shared {
        // TODO: Add resources
    }

    // Local resources go here
    #[local]
    struct Local {
        // TODO: Add resources
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::info!("init");

        // get the device from the context (cx)
        // and initiate all the needed ressources (led, button, clocks, timer ...)
        


        task1::spawn().ok();

        (
            Shared {
                // Initialization of shared resources go here
            },
            Local {
                // Initialization of local resources go here
            },
        )
    }

    // Background task, runs whenever no other tasks are running
    #[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::info!("idle");

        loop {
            continue;
        }
    }

    // TODO: Add tasks
    #[task] // Modifiy this line to add the interrupt
    async fn task1(_cx: task1::Context) {
        defmt::info!("Hello from task1!");
    }

    // TODO: Add tasks
    #[task] // Modifiy this line to add the interrupt
    async fn task2(_cx: task1::Context) {
        defmt::info!("Hello from task2!");
    }
}