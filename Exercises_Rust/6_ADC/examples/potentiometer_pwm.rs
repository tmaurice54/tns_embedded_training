// std and main are not available for bare metal software
#![no_main]
#![no_std]

// Panic handler
use panic_halt as _;

use core::fmt::Write;
use cortex_m_rt::entry;

// HAL library for stm32f4xx board
use stm32f4xx_hal::{
    pac,
    prelude::*,
    adc::{Adc, config::{AdcConfig, SampleTime}}, serial::{Serial, Config},
    timer::Channel,
};

// Programm entry
#[entry]
fn main() -> ! {

    // Get access to device and core peripherals
    let device = pac::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    // Get access to RCC 
    let rcc = device.RCC.constrain();
     
    let gpioa = device.GPIOA.split();

    // Set the sysclock and freeze it
    let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();

    // Set the ADC1 and PA0
    let mut adc = Adc::adc1(device.ADC1, true, AdcConfig::default());
    let pa0 = gpioa.pa0.into_analog();

    // Set up the LED pin
    let led = gpioa.pa5.into_alternate();

    // Create the pwm handler
    let mut led_pwm = device.TIM2.pwm_hz(led, 1000.Hz(),&clocks);

    // Get max duty
    let max_duty = led_pwm.get_max_duty();

    // set the duty
    led_pwm.enable(Channel::C1);

    // Set TX pin
    let tx_pin = gpioa.pa2.into_alternate();

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
    
    // Instantiate the delay
    let mut delay = core.SYST.delay(&clocks);

    loop {
        // Infitnite loop   

        // Get the digital value
        let digital_value = adc.convert(&pa0, SampleTime::Cycles_480);

        // Convert to correspond to PWM interval
        let new_duty = ((digital_value as f32/4096.0) *max_duty as f32) as u16;

        // Print on UART2
        writeln!(tx, "Digital value = {:}, Duty = {:}, Max duty = {:}\r", digital_value, new_duty, max_duty).unwrap();

        // Change the duty -> change the brightness 
        led_pwm.set_duty(Channel::C1, new_duty);

        delay.delay_ms(10u16);
    }
}
