# Exercise on GPIO

This first exercise introduces the usage of GPIOs.
You will control a led and a button.
In this exercise you will be using the STM32 HAL library which provides basic APIs to facilitate the utilization of your STM32 board.

Useful information:

The board STM32F401RE comes with a User LED (named LD2) connected to the STM32 I/O PA5, which means Port A Pin 5.
It also comes with a user button connected to the STM32 I/O PC13, which means Port C pin 13.

Timer2 Channel1 is linked to the user LED.

Useful links:  
[HAL Library](https://www.st.com/resource/en/user_manual/um1725-description-of-stm32f4-hal-and-lowlayer-drivers-stmicroelectronics.pdf) (Section 31.2 for GPIO)  
[Information about pins](https://os.mbed.com/platforms/ST-Nucleo-F401RE/)  
[Information about PWM and Timer on STM32](https://deepbluembedded.com/stm32-pwm-example-timer-pwm-mode-tutorial/)  

## Question 1: Turn on the User LED [Renode and Real Board]

The goal of this question is to write code that will turn on the User LED.  
To achieve that you will create a function in the main.cpp file that will turn on the User LED and then call this function in the main's infinite loop.

## Question 2: Blink the User LED [Renode and Real Board]

Create a function in the main file that will toggle the User LED.  
Then call this function every second in the main function, in the infinite loop.

## Question 3: Use button to turn on the LED [Renode and Real Board]

Create a function that will read the value of the button.
If the button is pressed, turn on LED and if the button is not pressed, turn off the LED.
Then call this function in the main function.

(Caution: On renode a button pressed mean 1 when it means 0 on the real board)

## Question 4: Use button with interruption mode [Renode and Real Board]

Same goal as for the previous question but now when the button is pressed it will create an interruption, so we don't need to permanently check if the button is pressed on the infinite loop.

As the button is pressed, the voltage of the pin will go to zero, so we need to detect a falling edge on the value of the voltage.
But we also need to know when the button is released to turn off the LEDs (rising edge).
The mode `GPIO_MODE_IT_RISING_FALLING` is what we need to achieve that.
Also, you will need to uncomment some code in the GPIO init function to allow the interruption on the button port/pin.
When an interruption is triggered, the MCU will execute a handler, which is called `HAL_GPIO_EXTI_IRQHandler`:

```cpp
void HAL_GPIO_EXTI_IRQHandler(uint16_t GPIO_Pin)
{
  /* EXTI line interrupt detected */
  if(__HAL_GPIO_EXTI_GET_IT(GPIO_Pin) != RESET)
  {
    __HAL_GPIO_EXTI_CLEAR_IT(GPIO_Pin);
    HAL_GPIO_EXTI_Callback(GPIO_Pin);
  }
}
```

This code will check that the interrupt has been triggered, clear the flag to make sure that the interrupt is not triggered continuously and then call our callback.
This callback can be user defined and we will redefine it in the main file.
In this callback, you will need to check the pin's value (SET or RESET) to work out which edge was triggered (falling or rising).
Write the callback that takes an uint16 as a parameter (the pin that triggered the interrupt) and makes the LEDs react accordingly.

## Question 5: Turn on LED with PWM [Real board]

In this question we will see the STM32 PWM generation and to achieve that we will use the STM32 timers.  
I recommend you read the documentation given in the useful links to understand everything about what PWM is and how it works.  

However, PWM consist of two main components: the Duty cycle and the Frequency.  
Duty cycle describes the amount of time, the signal is in HIGH state as a percentage of total time, it takes to complete one cycle.  
Frequency describes how fast the PWM completes a cycle and therefore how fast it switches between HIGH and LOW.  

You can find in the main file, in the `MX_TIM2_Init` function, that the Timer2 is already initiated with the appropriate values (You will have to call `MX_TIM2_Init` from the main function).
The prescaler of 72 will set the time clock of the timer to 1MHz as the original clock is 72MHz.  
And the period of 100 will set the timer's frequency to 10kHz.  
Now you can change the value of the `TIM2->CRR1` register which will change the duty cycle.  
`CRR1` stands for the channel 1, if you use another channel don't forget to change this register name (for example `TIM3->CRR2` if for channel 2 of timer 3).  
Here we use channel 1 because the channel 1 of the `Timer2` is connected to the User Led.
The value of this register must be between 0 and the period (100 here).  
For example, a value of 30 will make the duty cycle 30%.  

Now create a function that will change the Brightness of the user LED.  

## Question 6: LED Dimmer [Real board]

Now that you have understood what a PWM is and how to use it, try to change continually the brightness of the LED.  
You can create a loop which will slowly turn off the LED and after slowly turn on the LED.

## Question 7: servo motor [Real board]

PWM is also used to control motor.
In this question you have to use what you learn to control a servo motor.

The servo motor used in this training has 3 wire.
One brown for GND, one orange for PWM Input and one red for VCC.

Create a code that will change the position of the servo motor.
The servo motor has to begin at one of his extrem position, go to the over position in several seconds and comme back to the origin, and loop this execution.

For this question you will have to decomment and comment some lines in the `MX_TIM2_Init` function.
This is because the led and the servomotor don't use the same parameter for the `PWM input`.

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
Don't forget to initiate these pins, you can look at how this is done in the `MX_GPIO_Init` function and add code in this function or in the `main` funtion.

Try to connect the 7 segment display and display a counter from 0-9 that loops.
