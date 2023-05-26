# Exercise : Add C code in Rust programm

Many projects won't integrate Rust from scratch but will adopt gradually the language.
So it is important to learn how to add C/Rust code in a Rust/C programm.

To achieve that we will use the Foreign Function Interface (FFI) provided by Rust.
This interface is a way to call other languages in Rust and to call Rust in other languages.

We will also use a tool named bindgen which generates Rust FFI bindings to C/C++ libraries.
It takes as entry C/C++ header files and create code that can be used to call the C/C++ functions.

In this exercise we will see how to use FFI and bindgen to call the STM32 HAL library in Rust.

## Question 1: Compile the C/C++ project

The first thing to do is to create/write a STM32 C project.
I already did this using STM32CubeIDE.
You can find this project in the `c_project` directory.
Then we have to build the project, go in the `c_project` directory and execute:

```sh
mkdir build
make
```

This will build the project and put all generated files in the `build` directory.
You can see that the `main.c` file and `main` function have been renamed to `mainc`.
This is because we can't have 2 `main` functions in the whole project, and we want to use the `main` from Rust.

## Question 2: Create a static libray from the build

Now we have to generate a static library from the STM32 HAL C Project Output.
We will name this library `libstm32.a`.
The format has to be `.a` and the name must start with `lib` to be recognized.
For this we will use the `ar rcs` tool:

```sh
cd build
ar rcs libstm32.a gpio.o mainc.o stm32f4xx_hal_cortex.o stm32f4xx_hal_dma.o stm32f4xx_hal_dma_ex.o stm32f4xx_hal_exti.o stm32f4xx_hal_flash_ex.o stm32f4xx_hal_flash.o stm32f4xx_hal_gpio.o stm32f4xx_hal_msp.o stm32f4xx_hal_pwr.o stm32f4xx_hal_pwr_ex.o stm32f4xx_hal_rcc.o stm32f4xx_hal_rcc_ex.o stm32f4xx_hal_tim.o stm32f4xx_hal_tim_ex.o stm32f4xx_hal_uart.o stm32f4xx_it.o system_stm32f4xx.o usart.o stm32f4xx_hal.o
```

Now you can copy and paste that file in the directory `/rust_project/target/debug/deps` (You may need to create the folders `/target/debug/deps`) so it can be used in the rust project.


## Question 3: Create a wrapper.h

In the `rust_project` directory, create a new file `wrapper.h` and include all the headers containing declarations we want to bind.
I already create a folder `cheaders` with all the headers files needed.
You just have to create `wrapper.h` and write:

```c
#include "cheaders/stm32f4xx.h"
#include "cheaders/system_stm32f4xx.h"
#include "cheaders/stm32f401xe.h"

#include "cheaders/core_cm4.h"
#include "cheaders/cmsis_version.h"
#include "cheaders/cmsis_compiler.h"
#include "cheaders/cmsis_gcc.h"
#include "cheaders/mpu_armv7.h"

#include "cheaders/stm32f4xx_hal_conf.h"
#include "cheaders/stm32f4xx_it.h"
#include "cheaders/mainc.h"
#include "cheaders/gpio.h"
#include "cheaders/usart.h"

#include "cheaders/stm32f4xx_hal.h"
#include "cheaders/stm32f4xx_hal_rcc_ex.h"
#include "cheaders/stm32f4xx_hal_rcc.h"
#include "cheaders/stm32f4xx_hal_def.h"
#include "cheaders/Legacy/stm32_hal_legacy.h"
#include "cheaders/stm32f4xx_hal_gpio.h"
#include "cheaders/stm32f4xx_hal_gpio_ex.h"
#include "cheaders/stm32f4xx_hal_exti.h"
#include "cheaders/stm32f4xx_hal_dma.h"
#include "cheaders/stm32f4xx_hal_dma_ex.h"
#include "cheaders/stm32f4xx_hal_cortex.h"
#include "cheaders/stm32f4xx_hal_flash.h"
#include "cheaders/stm32f4xx_hal_flash_ex.h"
#include "cheaders/stm32f4xx_hal_flash_ramfunc.h"
#include "cheaders/stm32f4xx_hal_pwr.h"
#include "cheaders/stm32f4xx_hal_pwr_ex.h"
#include "cheaders/stm32f4xx_hal_uart.h"
```

## Question 4: Create build.rs file

Create a file `build.rs` in the `rust_project` directory and add:

```rust
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rustc-link-lib=static=stm32");

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory.x");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .use_core()
        .ctypes_prefix("cty")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
```

This file will be automatically used during the build.
`build.rs` essentially allows to automate custom builds.
In our case, we need to generate Rust FFi bindings for all the headers we included in `wrapper.h`.

## Question 5: Integrate the needed functions

In the `main.rs` file, we can now include the statements:

```rust
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs")); 
```

And now you can use the STM32 HAL functions in your rust programm.
Try to toggle the led and use the button.

Here an example of a programm that blink the led every second:

```rust
const GPIOA: *mut GPIO_TypeDef = GPIOA_BASE as *mut GPIO_TypeDef;
const GPIO_PIN_5: u16 = 0x0020;

#[entry]
fn main() -> ! {
    // Application Loop
    // Setup handler for device peripherals
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

    let mut del = dp.TIM1.delay_ms(&clocks);

    let gpioa = dp.GPIOA.split();
    let _led = gpioa.pa5.into_push_pull_output();

    loop {
        // Call C function in bindings.rs that toggles pin
        unsafe {
            HAL_GPIO_TogglePin(GPIOA, GPIO_PIN_5);
        }
        del.delay_ms(1000_u32);
    }
}
```
