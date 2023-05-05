# Exercise on GPIO

This first exercise introduces the usage of GPIOs using Rust language.
You will control a led and a button.
In this exercise you will be using the STM32 HAL library which provides basic APIs to facilitate the utilization of your STM32 board.

Useful informatons :

The board STM32F401RE come with a User Led (named led 2) connected to the STM32 I/O PA5, wich means Port A Pin 5.
It also come with a User button connected to the STM32 I/O PC13, wich means Port B pin 13.

The user LED is linked to the Timer2 Channel1

Useful links :  
[Crate stm32f4xx_hal](https://docs.rs/stm32f4xx-hal/latest/stm32f4xx_hal/)  
[Information about pin](https://os.mbed.com/platforms/ST-Nucleo-F401RE/)  
[Rust embedded doc](https://docs.rust-embedded.org/book/intro/index.html/)  
[Page with multiple project](https://dev.to/apollolabsbin/)  

## Question 1 : Turn on the User LED [Renode and Real Board]

The goal of this question is to create a code that will turn on the User LED.  
To achieve the first step is to set you LED with this code :

```rust
    //  Get access to GPIOA
    let gpioa = device.GPIOA.split();

    // Set up the LED pin
    let mut led = gpioa.pa5.into_push_pull_output();
```

Now you can turn on the LED with the `.set_high()` functions.

## Question 2 : Blink the User LED [Renode and Real Board]

Now create code that will toggle the LED every second.
To achieve that you will need to create a delay handler :

```rust
    // Set up systick delay
    let mut delay = core.SYST.delay(&clocks);
```

You can now create a delay with the functions `.delay_ms()`.
You may also need the function `.toggle()` to toggle the LED.

## Question 3 : Use button to turn on the LED [Renode and Real Board]

Create a code that will read the value of the button.
If the button is pressed, turn on LED and if the button is not pressed, turn off the LED.

You may need the functions `is_high()` and/or `is_low()`;

(Caution : On renode a button pressed mean 1 when it actually mean 0 on the real board)

## Question 4 : Use button with interruption mode [Renode and Real Board]

TO DO

## Question 5 : Turn on LED with PWM [Real board]

Create a function that will turn on the LED in a PWM mode.
You can change the brigthness as you want modifing the duty cycle.
To achieve that you will need the following code :

```rust
    // Set up the LED pin into alternate mode
    let pin = gpiox.pax.into_alternate();

    // Create the pwm handler with a frequence of 2000Hz (example)
    let mut pin_pwm = device.TIMX.pwm_hz(pin, 2000.Hz(), &clocks);
    
    // Get max duty
    let mut max_duty = 65535;

    // Enable the pwm on channel C1
    pin_pwm.enable(Channel::CX);

    // Set the duty of the pin
    pin_pwm.set_duty(Channel::CX, max_duty);
```

Don't forget that the LED is linked to the `TIM2`, and you can use the channel 1.

## Question 6 : LED Dimmer [Real board]

Now that you have understand what is PWM  and how to use it.  
Try to change continually the brightness of the LED.  
You can create a loop which will slowly turn off the LED and after slowly turn on the LED.
