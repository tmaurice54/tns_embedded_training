// std and main are not available for bare metal software
#![no_main]
#![no_std]

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
// Panic handler
use panic_halt as _;

use cortex_m_rt::entry;

// HAL library for stm32f4xx board
use stm32f4xx_hal::{
    pac::{self, interrupt},
    gpio::{self, Edge, Input, Output},
    prelude::*,
};

// Types for button and led
type ButtonPin = gpio::PC13<Input>;
type LedPin = gpio::PA5<Output>;

// Wrap the ButtonPin
static G_BUTTON: Mutex<RefCell<Option<ButtonPin>>> = Mutex::new(RefCell::new(None));
// Wrap the Led pin
static G_LED: Mutex<RefCell<Option<LedPin>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {

    // Get access to device and core peripherals
    let mut device = pac::Peripherals::take().unwrap();
    let _core: cortex_m::Peripherals = cortex_m::Peripherals::take().unwrap();

    // Get access to RCC 
    let rcc = device.RCC.constrain();
    // Set the sysclock and freeze it
    let _clocks = rcc.cfgr.use_hse(8.MHz()).freeze();
    // Constrains SYSCFG
    let mut syscfg = device.SYSCFG.constrain();

    // Get access to GPIOA and GPIOB
    let gpioa = device.GPIOA.split();
    let gpioc = device.GPIOC.split();

    // Set up the LED and button pins
    let led = gpioa.pa5.into_push_pull_output();
    let mut button = gpioc.pc13;
    
    // Create and allow interrupt on button
    button.make_interrupt_source(&mut syscfg);    
    button.trigger_on_edge(&mut device.EXTI, Edge::RisingFalling);
    button.enable_interrupt(&mut device.EXTI);

    // Enable the external interrupt
    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::EXTI15_10);
    }

    // Move button and led to global contex 
    cortex_m::interrupt::free(|cs| {
        G_BUTTON.borrow(cs).replace(Some(button));
        G_LED.borrow(cs).replace(Some(led));
    });

    loop {

    }
}

#[interrupt]
fn EXTI15_10() {
    cortex_m::interrupt::free(|cs| {
        let mut led = G_LED.borrow(cs).borrow_mut();
        led.as_mut().unwrap().toggle();
        let mut button = G_BUTTON.borrow(cs).borrow_mut();
        button.as_mut().unwrap().clear_interrupt_pending_bit();
    });
}