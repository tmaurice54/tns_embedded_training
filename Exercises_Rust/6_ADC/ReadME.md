# Exercise on ADC

ADC stands for Analog to Digital Converter, it is a peripheral used to convert an electrical signal (analog voltage) to a binary value.
In this exercise we will work with a potentiometer and read his value with the ADC.

A potentiometer is a three-terminal resistor with a sliding or rotating contact that forms an adjustable voltage divider (Wikipedia).

So potentiometer is a classic resistor with a third terminal which will be connected to the ADC, and depend on the position of the potentiometer, the read value change.

Informations:

- ADC1 is on PA0

Links:  
[Crate stm32f4xx_hal](https://docs.rs/stm32f4xx-hal/latest/stm32f4xx_hal/)  
[Information about pin](https://os.mbed.com/platforms/ST-Nucleo-F401RE/)  
[Rust embedded doc](https://docs.rust-embedded.org/book/intro/index.html/)  

## Question 1: Read value from ADC

For the connection with the board you can connect the left terminal to the `GND`, the middle one to PA0 and the right one to `3V3`.
Once you have connected your potentiometer to the board, you will have to read his value.
You can use the following code to read value from `ADC1` and save it in a variable:

```rust
// Set the adc1
let mut adc = Adc::adc1(device.ADC1, true, AdcConfig::default());

// Set the pin PA0
let pa0 = gpioa.pa0.into_analog();

// Get the digital value
let digital_value = adc.convert(&pa0, SampleTime::Cycles_480);

// Get the millivolts value (if needed)
let millivolts = adc.sample_to_millivolts(digital_value);
```

## Question 2: Change the brightness of the Led

Create a code that will change the brightness of the user led (or another one) depends on the position of the potentiometer.
You will have to decomment some code in the `main` and in the `MX_TIM2_Init` functions. (Leave commented if not used)  

Don't forget to start the `PWM Timer2` on the channel 1 for the User Led, and to calculate the good value depend on the `max_duty` of the PWM.

## Question 3: Control the servomotor

TODO
