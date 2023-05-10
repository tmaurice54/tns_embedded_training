// std and main are not available for bare metal software
#![no_main]
#![no_std]

// Macro to write data
use core::fmt::Write;
// Panic handler
use panic_halt as _;

use cortex_m_rt::entry;

// HAL library for stm32f4xx board
use stm32f4xx_hal::{
    pac,
    prelude::*,
    flash::LockedFlash,
    serial::{config::Config, Serial},
};

use embedded_storage::nor_flash::ReadNorFlash;


#[entry]
fn main() -> ! {

    // Get access to device and core peripherals
    let device = pac::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();
    
    let mut flash = LockedFlash::new(device.FLASH);

    // Get access to RCC 
    let rcc = device.RCC.constrain();
    // Set the sysclock and freeze it
    let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();

    //  Get access to GPIOA
    let gpioa = device.GPIOA.split();

    // Set up the LED pin
    let mut led = gpioa.pa5.into_push_pull_output();

    // Set up systick delay
    let mut delay = core.SYST.delay(&clocks);

    // Set TX pin
    let tx_pin = gpioa.pa2.into_alternate();

    let mut buf = [0u8; 4];
    ReadNorFlash::read(&mut flash, 0x1E900, &mut buf).unwrap();

    // Set UART2
    let mut tx = Serial::tx(
        device.USART2,
        tx_pin,
        Config::default()
                .baudrate(115200.bps())
                .wordlength_8()
                .parity_none(),
        &clocks,
        )
        .unwrap();
    
    loop {
        writeln!(tx, "{0}{1}{2}{3}\r",buf[0] as char,buf[1] as char,buf[2] as char,buf[3] as char).unwrap();
        led.toggle();
        delay.delay_ms(1000u16);
    }
}