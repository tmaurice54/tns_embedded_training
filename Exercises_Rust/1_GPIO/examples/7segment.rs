// std and main are not available for bare metal software
#![no_main]
#![no_std]

// Panic handler
use panic_halt as _;
use cortex_m_rt::entry;

// HAL library for stm32f4xx board
use stm32f4xx_hal::{
    pac,
    prelude::*,
};

#[entry]
fn main() -> ! {
    // Get access to device and core peripherals
    let device = pac::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    // Get access to RCC 
    let rcc = device.RCC.constrain();
    // Set the sysclock and freeze it
    let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();

    //  Get access to GPIOA
    let gpioa = device.GPIOA.split();

    // Set up the LED pin
    let mut a = gpioa.pa0.into_push_pull_output();
    let mut b = gpioa.pa1.into_push_pull_output();
    let mut c = gpioa.pa4.into_push_pull_output();
    let mut d = gpioa.pa5.into_push_pull_output();
    let mut e = gpioa.pa6.into_push_pull_output();
    let mut f = gpioa.pa7.into_push_pull_output();
    let mut g = gpioa.pa9.into_push_pull_output();
   
    // Set up systick delay
    let mut delay = core.SYST.delay(&clocks);
    let mut count = 0;
    loop {
        match count {
            0 => {a.set_low();b.set_low();c.set_low();d.set_low();e.set_low();f.set_low();g.set_high()},
            1 => {a.set_high();b.set_low();c.set_low();d.set_high();e.set_high();f.set_high();g.set_high()},
            2 => {a.set_low();b.set_low();c.set_high();d.set_low();e.set_low();f.set_high();g.set_low()},
            3 => {a.set_low();b.set_low();c.set_low();d.set_low();e.set_high();f.set_high();g.set_low()},
            4 => {a.set_high();b.set_low();c.set_low();d.set_high();e.set_high();f.set_low();g.set_low()},
            5 => {a.set_low();b.set_high();c.set_low();d.set_low();e.set_high();f.set_low();g.set_low()},
            6 => {a.set_low();b.set_high();c.set_low();d.set_low();e.set_low();f.set_low();g.set_low()},
            7 => {a.set_low();b.set_low();c.set_low();d.set_high();e.set_high();f.set_high();g.set_high()},
            8 => {a.set_low();b.set_low();c.set_low();d.set_low();e.set_low();f.set_low();g.set_low()},
            9 => {a.set_low();b.set_low();c.set_low();d.set_low();e.set_high();f.set_low();g.set_low()},
            _ => ()
        }
        count=(count+1)%10;
        delay.delay_ms(1000u16);
    }
}