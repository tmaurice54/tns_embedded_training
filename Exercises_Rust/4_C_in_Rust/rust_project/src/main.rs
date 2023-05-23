#![no_std]
#![no_main]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Imports
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{self, pac, prelude::*};

// Include bindings.rs that contains the extern function declatrations
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

const GPIOA: *mut GPIO_TypeDef = GPIOA_BASE as *mut GPIO_TypeDef;
const GPIOC: *mut GPIO_TypeDef = GPIOC_BASE as *mut GPIO_TypeDef;

// alternative declaration
// const GPIOA: *mut GPIO_TypeDef = GPIOA_BASE as *mut _;
const GPIO_PIN_5: u16 = 0x0020;
const GPIO_PIN_13: u16 = 0x2000;


#[entry]
fn main() -> ! {
    // Application Loop
    // Setup handler for device peripherals
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let _clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

    //let mut del = dp.TIM1.delay_ms(&clocks);

    let gpioa = dp.GPIOA.split();
    let gpioc = dp.GPIOC.split();

    let _led = gpioa.pa5.into_push_pull_output();
    let _button = gpioc.pc13;

    loop {
        // Call C function in bindings.rs that toggles pin
        unsafe {
            if HAL_GPIO_ReadPin(GPIOC, GPIO_PIN_13) == 0 {
                HAL_GPIO_WritePin(GPIOA, GPIO_PIN_5, 1);
              } else {
                HAL_GPIO_WritePin(GPIOA, GPIO_PIN_5, 0);
              }
        }
        // del.delay_ms(200_u32);
    }
}
