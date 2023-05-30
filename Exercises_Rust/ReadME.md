# Exercises Rust

Here you will find the C/C++ exercises of the embedded training.
For now there is 4 different exercises that you can do.

## The exercises

### Hello World

[This exercise](./0_HelloWorld/) is an intiation about how to create a Rust project and how to build it.
It is not specialized about embedded project.

### GPIO

[This exercise](./1_GPIO/) is about how to use the GPIO using a STM32 board.
You will learn how to turn on/off leds, how to use button to trigger action, interruption on EXTI and PWM.

### Sensor

[This exercise](./2_Sensor/) is about SPI sensor.
You will learn how to read the temperature from a temperature sensor.

### Linker Script

[This exercise](./3_LinkerScript/) is about the linker scripts.
You will learn how to find a signature, how to read it and how to modify it.

### C in Rust

[This exercise](./4_C_in_Rust/) is about adding C/C++ in Rust programms.
You will learn how to create a lib from a C/C++ project and how to use it in a Rust project.

### RTIC

[This exercise](./5_RTIC/) is about the framework RTIC.
You will learn hoy to create tasks controlled by interruption (EXTI, TIMER, ...).

## How to build, flash and debug

You can find all informations needed [here](../Docs/rust_configuration.md).

## How to use Renode

You have [here](../Docs/use_renode_robotframework.md) all the informations needed.
