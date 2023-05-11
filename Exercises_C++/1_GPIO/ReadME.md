# Exercise on GPIO

This first exercise introduces the usage of GPIOs.
You will control a led and a button.
In this exercise you will be using the STM32 HAL library which provides basic APIs to facilitate the utilization of your STM32 board.

Useful information:

The board STM32F401RE come with a User Led (named led 2) connected to the STM32 I/O PA5, which means Port A Pin 5.
It also come with a user button connected to the STM32 I/O PC13, which means Port B pin 13.

Timer2 Channel1 is linked to the user LED.

Useful links:  
[HAL Library](https://www.st.com/resource/en/user_manual/um1725-description-of-stm32f4-hal-and-lowlayer-drivers-stmicroelectronics.pdf) (Section 31.2 for GPIO)  
[Information about pin](https://os.mbed.com/platforms/ST-Nucleo-F401RE/)  
[Information about PWM and Timer on STM32](https://deepbluembedded.com/stm32-pwm-example-timer-pwm-mode-tutorial/)  

## Question 1: Turn on the User LED [Renode and Real Board]

The goal of this question is to create a code that will turn on the User LED.  
To achieve that you will create a function in the main file that will turn on the User LED and then call this function in the main function, in the infinite loop.

## Question 2: Blink the User LED [Renode and Real Board]

Create a function in the main file that will Toggle the User LED.  
Then call this function every second in the main function, in the infinite loop.

## Question 3: Use button to turn on the LED [Renode and Real Board]

Create a function that will read the value of the button.
If the button is pressed, turn on LED and if the button is not pressed, turn off the LED.
Then call this function in the main function.

(Caution: On renode a button pressed mean 1 when it means 0 on the real board)

## Question 4: Use button with interruption mode [Renode and Real Board]

Same goal than for the previous question but now when the button is pressed it will create an interruption, so we don't need to permanently check if the button is pressed on the infinite loop.

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

You can find in the main file, in the `MX_TIM2_Init` function, that the Timer2 is already initiate with good values (You will have to call this function in the main function).
The prescaler of 72 will set the time clock of the timer to 1MHz as the original clock is 72MHz.  
And the period of 100 will set the frequency of to 10kHz.  
Now you can change the value of the TIM2->CRR1 register which will change the duty cycle.  
The value of this register must be between 0 and the period (100 here).  
For example, a value of 30 will make the duty cycle to 30%.  

Now create a function that will change the Brightness of the user LED.  

## Question 6: LED Dimmer [Real board]

Now that you have understand what PWM is and how to use it.  
Try to change continually the brightness of the LED.  
You can create a loop which will slowly turn off the LED and after slowly turn on the LED.
