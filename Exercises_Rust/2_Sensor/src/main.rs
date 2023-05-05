// std and main are not available for bare metal software
#![no_main]
#![no_std]

// Panic handler
use panic_halt as _;
use core::fmt::Write;
use cortex_m_rt::entry;

// HAL library for stm32f4xx board
use stm32f4xx_hal::{
    spi::{Mode, NoMosi, Phase, Polarity, Spi},
    pac,
    prelude::*,
    serial::{config::Config, Serial}
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
    let gpiob = device.GPIOB.split();

    // Set sclk, cs and miso for spi protocol
    let sclk = gpiob.pb10.into_alternate();
    let miso = gpiob.pb14.into_alternate();
    let mut cs = gpiob.pb12.into_push_pull_output();
    // Set the led pin
    let mut led = gpioa.pa5.into_push_pull_output();
    // Set TX pin
    let tx_pin = gpioa.pa2.into_alternate();

    // Set SPI2
    let mut spi = Spi::new(
        device.SPI2, 
        (sclk, miso, NoMosi{}), 
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        }, 
        2.MHz(), 
        &clocks);

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

    // Set delay
    let mut delay = core.SYST.delay(&clocks);

    let mut words = [0x00,0x00];

    loop {
        cs.set_low();
        match spi.transfer(&mut words){
            Ok(sk) => { 
                let mut number = ((sk[0] as u16) << 8) | sk[1] as u16;
                if number > 0x7000 {
                    number = number & 0x7FFF;
                }
                let temp = ((number>>3)as f32*0.0625) as i16;
                writeln!(tx, "{}\r", temp).unwrap();
                if temp == 50 {
                    led.set_high();
                } else {
                    led.set_low();
                }
            },
            Err(_e) => {
                
            },
        }
        cs.set_high();
        delay.delay_ms(1000u16);
    }
}