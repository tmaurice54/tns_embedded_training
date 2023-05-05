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
    timer::Channel,
};
#[entry]
fn main() -> ! {

    // Get access to device and core peripherals
    let device = pac::Peripherals::take().unwrap();
    let _core = cortex_m::Peripherals::take().unwrap();
    // Get access to RCC 
    let rcc = device.RCC.constrain();
    // Set the sysclock and freeze it
    let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();

    //  Get access to GPIOA
    let gpioa = device.GPIOA.split();

    // Set up the LED pin
    let led = gpioa.pa5.into_alternate();

    // Create the pwm handler
    let mut led_pwm = device.TIM2.pwm_hz(led, 2000.Hz(),&clocks);

    // Get max duty
    let max_duty = led_pwm.get_max_duty();

    // set the duty
    led_pwm.enable(Channel::C1);

    loop {
        led_pwm.set_duty(Channel::C1, max_duty/10);
    }
}