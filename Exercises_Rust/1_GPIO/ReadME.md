# Exercise on GPIO

This first exercise introduces the usage of GPIOs using the Rust language.
You will control a led and a button.
In this exercise you will be using the STM32 HAL library which provides basic APIs to facilitate the utilization of your STM32 board.

Useful information:

The board STM32F401RE come with a User Led (named led 2) connected to the STM32 I/O PA5, which means Port A Pin 5.
It also come with a user button connected to the STM32 I/O PC13, which means Port B pin 13.

The user led is linked to the Timer 2 Channel 1

Useful links:  
[Crate stm32f4xx_hal](https://docs.rs/stm32f4xx-hal/latest/stm32f4xx_hal/)  
[Information about pin](https://os.mbed.com/platforms/ST-Nucleo-F401RE/)  
[Rust embedded doc](https://docs.rust-embedded.org/book/intro/index.html/)  
[Page with multiple project](https://dev.to/apollolabsbin/)  

## Question 1: Turn on the user led [Renode and Real Board]

The goal of this question is to create a code that will turn on the user led.  
To achieve the first step is to set your led with this code:

```rust
    //  Get access to GPIOA
    let gpioa = device.GPIOA.split();

    // Set up the led pin
    let mut led = gpioa.pa5.into_push_pull_output();
```

Now you can turn on the led with the `.set_high()` functions.

## Question 2: Blink the User led [Renode and Real Board]

Now create code that will toggle the led every second.
To achieve that you will need to create a delay handler:

```rust
    // Set up systick delay
    let mut delay = core.SYST.delay(&clocks);
```

You can now create a delay with the functions `.delay_ms()`.
You may also need the function `.toggle()` to toggle the led.

## Question 3: Use button to turn on the led [Renode and Real Board]

Create a code that will read the value of the button.
If the button is pressed, turn on the led and if the button is not pressed, turn off the led.

You may need the functions `is_high()` and/or `is_low()`;

(Caution: On renode a button pressed mean 1 when it means 0 on the real board)

## Question 4: Use button with interruption mode [Renode and Real Board]

Write code that will turn on the led when we press the button and turn off the led when we release it.

To create an interrupt with a button and a led we must create static global variable called G_BUTTON and G_LED as follows:  

```rust
// Create types to simplify the syntax
type ButtonPin = gpio::PC13<Input>;
type LedPin = gpio::PA5<Output>;

// Wrap the ButtonPin
static G_BUTTON: Mutex<RefCell<Option<ButtonPin>>> = Mutex::new(RefCell::new(None));
// Wrap the Led pin
static G_LED: Mutex<RefCell<Option<LedPin>>> = Mutex::new(RefCell::new(None));
```

So here the peripheral is wrapped in an `Option` that is wrapped in a `RefCell`, that is wrapped in a `Mutex`.  
The `Mutex` makes sure that the peripheral can be safely shared among threads.  
It would require that we use a critical section to be able to access the peripheral.  
The `RefCell` is used to be able obtain a mutable reference to the peripheral.  Finally, the `Option` is used to allow for lazy initialization as one would not be able to initialize the variable until later.  

Next, we have to create and allow the interrupt on the button.  
Here we want an interrupt on rising and falling edge:  

```rust
    // Create and allow interrupt on button
    button.make_interrupt_source(&mut syscfg);    
    button.trigger_on_edge(&mut device.EXTI, Edge::RisingFalling);
    button.enable_interrupt(&mut device.EXTI);

    // Enable the external interrupt
    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::EXTI15_10);
    }
```

We also have to move the led and the button to global context:  

```rust
    // Move button and led to global contex 
    cortex_m::interrupt::free(|cs| {
        G_BUTTON.borrow(cs).replace(Some(button));
        G_LED.borrow(cs).replace(Some(led));
    });
```

And we must create the interrupt service routine:  

```rust
#[interrupt]
fn EXTI15_10() {
    cortex_m::interrupt::free(|cs| {
        let mut pin = G_PIN.borrow(cs).borrow_mut();
        pin.as_mut().unwrap().some_function()
    });
}
```

You may need the function `.clear_interrupt_pending_bit()` to clear the interrupt on the button.

If you need more help, check this [link](https://dev.to/apollolabsbin/stm32f4-embedded-rust-at-the-hal-gpio-interrupts-e5)

## Question 5: Turn on led with PWM [Real board]

Create a function that will turn on the led in a PWM mode.
You can change the brightness as you want modifying the duty cycle.
To achieve that you will need the following code:

```rust
    // Set up the led pin into alternate mode
    let pin = gpiox.pax.into_alternate();

    // Create the pwm handler with a frequency of 2000Hz (example)
    let mut pin_pwm = device.TIMX.pwm_hz(pin, 2000.Hz(), &clocks);
    
    // Get max duty
    let mut max_duty = led_pwm.get_max_duty();

    // Enable the pwm on channel CX
    pin_pwm.enable(Channel::CX);

    // Set the duty of the pin
    pin_pwm.set_duty(Channel::CX, max_duty);
```

Don't forget that the led is linked to the `TIM2`, and you can use the channel 1.

## Question 6: led Dimmer [Real board]

Now that you have understood what a PWM is and how to use it, try to change continually the brightness of the led.  
You can create a loop which will slowly turn off the led and after slowly turn on the led.

## Question 7: servo motor [Real board]

PWM is also used to control motor.
In this question you have to use what you learn to control a servo motor.

The servo motor used in this training has 3 wire.
One brown for GND, one orange for PWM Input and one red for VCC.

Create a code that will change the position of the servo motor.
The servo motor has to begin at one of his extrem position, go to the over position in several seconds and comme back to the origin, and loop this execution.

For this question you will have to change the frequency used to 50hz.

Information:

A servo motor is a specific motor with precision and a defined ranged of motion.
We can control them with `PWM`, usually they work with 50Hz input and a value of 1 for 1ms means one extreme (0 degree) and 2ms means the other extreme (180 degree).
But there is a lot of different servomotor and you have to look at the datasheet to have the full configuration.

[Video about servomotor](https://www.youtube.com/watch?v=g68khnZnJKM&ab_channel=Thescienceworks)

## Question 8: 7 Segment display [Real Board]

In this training you have normaly access to 7 segment display.
They are used to diplay one digit 0-9.
There is two type of those, cathode or anode, and in this training we will focus on the second one.
But the differenc is not big, just the intern circuit and so the connection with the board are different.

![7 segment](../../Docs/ressources/7Segment.jpg)

To connect this 7 segment display to the board, you can connect one of the Vcc to the 5V output of the board with a 1kOhm resistor.
Then connect the a,b,c,d,e,f,g,dp to the pin of the board.
You can chose any GPIO pins, but some pins are not avalable because of other periphral or something else.
For example you can use the pins PA0,PA1,PA4,PA5,PA6,PA7 and PA9.  
Don't forget to initiate these pins in output mode.

Try to connect the 7 segment display and display a counter from 0-9 that loops.
