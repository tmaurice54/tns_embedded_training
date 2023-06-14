#![no_std]
#![no_main]

// Imports
use core::fmt::Write;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{
    serial::{config::Config, Serial},
    i2c::{Mode,I2c},
    pac::{self},
    prelude::*,
};

#[entry]
fn main() -> ! {
    // Setup handler for device peripherals
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();
    // 2) Configure/Define SCL and SDA pins
    let gpioc = dp.GPIOC.split();
    let gpioa = dp.GPIOA.split();
    let scl = gpioa.pa8;
    let sda = gpioc.pc9;

    let mut i2c = I2c::new(
        dp.I2C3,
        (scl, sda),
        Mode::Standard {
            frequency: 100.kHz(),
        },
        &clocks,
    );

    // Set TX pin
    let tx_pin = gpioa.pa2.into_alternate();

    // Set UART2
    let mut tx = Serial::tx(
        dp.USART2,
        tx_pin,
        Config::default()
                  .baudrate(115200.bps())
                  .wordlength_8()
                  .parity_none(),
        &clocks,
        )
      .unwrap();

    let mut delay = dp.TIM1.delay_ms(&clocks);

    const TC74_ADDR: u8 = 0x4A;

    let mut rx_buffer: [u8; 2] = [0; 2];


    // Application Loop
    loop {
        match i2c.write(TC74_ADDR, &[0x00]) {
            Err(_) => writeln!(tx, "Err Tx\r").unwrap(),
            Ok(_) => ()
        }
        match i2c.read(TC74_ADDR, &mut rx_buffer)  {
            Err(_) => writeln!(tx, "Err Rx\r").unwrap(),
            Ok(_) => ()
        }
        // Print Temperature Value
        writeln!(tx, "Temperature = {:}°C\r", rx_buffer[0]).unwrap();
        delay.delay_ms(1000u16);
    }
}