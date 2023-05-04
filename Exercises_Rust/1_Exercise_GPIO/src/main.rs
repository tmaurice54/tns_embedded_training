// std and main are not available for bare metal software
#![no_main]
#![no_std]

// Panic handler
use panic_halt as _;

// Use to define the entry of the programm
use cortex_m_rt::entry;

// HAL library for stm32f4xx board
use stm32f4xx_hal::{
    pac,
    prelude::*
};

// Programm entry
#[entry]
fn main() -> ! {

    // Get access to device and core peripherals
    let device = pac::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    // Get access to RCC 
    let rcc = device.RCC.constrain();
    // Set the sysclock and freeze it
    let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();
    
    loop {
        // Infitnite loop
    }
}