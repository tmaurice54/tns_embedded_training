#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Input, Pull};
use {defmt_rtt as _, panic_probe as _};
use embassy_stm32::gpio::{Level, Output,Speed};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let button = Input::new(p.PC13, Pull::Down);
    let mut button = ExtiInput::new(button, p.EXTI13);

    info!("Press the USER button...");
    let mut led = Output::new(p.PA5, Level::Low, Speed::Low);

    loop {
        button.wait_for_rising_edge().await;
        led.set_low();
        info!("Pressed!");
        button.wait_for_falling_edge().await;
        info!("Released!");
        led.set_high();
    }
}