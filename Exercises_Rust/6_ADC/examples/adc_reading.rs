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

    let mut adc = Adc::adc1(device.ADC1, true, AdcConfig::default());

    let pa0 = gpioa.pa0.into_analog();

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
    
    let mut delay = core.SYST.delay(&clocks);

    loop {
        // Infitnite loop   
        let digital_value = adc.convert(&pa0, SampleTime::Cycles_480);
        let millivolts = adc.sample_to_millivolts(digital_value);
        writeln!(tx, "Millivolt = {:}, Digital value = {:}\r",millivolts, digital_value).unwrap();
        delay.delay_ms(10u16);
    }
}
