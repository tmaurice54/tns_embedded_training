// std and main are not available for bare metal software
#![no_main]
#![no_std]

// Panic handler
use panic_halt as _;
use cortex_m_rt::entry;


// HAL library for stm32f4xx board
use stm32f4xx_hal::{
    spi::{Mode, Phase, Polarity, Spi, Instance},
    pac,
    prelude::*,
};

struct Sensor<SPI:Instance, PIN> { 
    spi:Spi<SPI, PIN>,
}

impl<SPI:Instance,Ouput> Sensor<SPI, Ouput> {
    fn get_temp(&mut self) -> i32{
        let mut words = [0x00,0x00];
        match self.spi.transfer( &mut words){
            Ok(buff) => { 
                let mut number = ((buff[0] as i16) << 8) | buff[1] as i16;
                if number > 0x7000 {
                    number = number & 0x7FFF;
                }
                return (((number>>3)as f32)*0.0625) as i32;
            },
            Err(_) => {
                return -1;
            },
        }
    }
}

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
    let mosi = gpiob.pb15.into_alternate();
    let mut cs = gpiob.pb12.into_push_pull_output();
    // Set the led pin
    let mut led = gpioa.pa5.into_push_pull_output();

    // Set SPI2
    let spi = Spi::new(
        device.SPI2, 
        (sclk, miso, mosi), 
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        }, 
        2.MHz(), 
        &clocks);

    // Set delay
    let mut delay = core.SYST.delay(&clocks);
    let mut sensor = Sensor{spi};
    let mut temp;

    loop {
        cs.set_low();
        temp = sensor.get_temp();
        cs.set_high();
        if temp == 50 {
            led.set_high();
        } else {
            led.set_low();
        }
        delay.delay_ms(1000u16);
    }
}
