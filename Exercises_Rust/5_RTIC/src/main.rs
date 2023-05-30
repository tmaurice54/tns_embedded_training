#![no_main]
#![no_std]

use panic_halt as _;

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
    fn init(cx: init::Context) -> (Shared, Local,init::Monotonics()) {

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
            // Move the monotonic timer to the RTIC run-time, this enables
            // scheduling
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

    #[task] // Modifiy this line to add the interrupt
    async fn task1(_cx: task1::Context) {
        
    }

    #[task] // Modifiy this line to add the interrupt
    async fn task2(_cx: task1::Context) {

    }
}