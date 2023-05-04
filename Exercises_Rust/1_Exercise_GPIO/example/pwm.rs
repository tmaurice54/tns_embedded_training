// std and main are not available for bare metal software
#![no_main]
#![no_std]

pub use embedded_hal as hal;

// Panic handler
use panic_halt as _;

use cortex_m_rt::entry;

// HAL library for stm32f4xx board
use stm32f4xx_hal::{
    pac,
    prelude::*,
    timer::Channel
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
    let led = gpioa.pa5.into_alternate();

    // Create a Timer

    // Create the pwm handler
    let mut led_pwm = device.TIM2.pwm_hz(led, 2000.Hz(),&clocks);
    
    // Get max duty
    let mut max_duty = 65535;

    // set the duty
    led_pwm.enable(Channel::C1);
    led_pwm.set_duty(Channel::C1, max_duty);
    
    let mut delay = core.SYST.delay(&clocks);

    loop {
        while max_duty > 0 {
            led_pwm.set_duty(Channel::C1, max_duty);
            max_duty -= 5;
            delay.delay_ms(1u16);
        }
        while max_duty < 65535 {
            led_pwm.set_duty(Channel::C1, max_duty);
            max_duty += 5;
            delay.delay_ms(1u16);
        }
    }
}