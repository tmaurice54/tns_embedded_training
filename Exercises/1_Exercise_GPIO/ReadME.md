# Exercise on GPIO

This first exercise introduces the usage of GPIOs.
You will control a led and a button.
In this exercise you will be using the STM32 HAL library which provides basic APIs to facilitate the utilization of your STM32 board.

Useful informatons :

The board STM32F401RE come with a User Led (named led 2) connected to the STM32 I/O PA5, wich means Port A Pin 5.
It also come with a User button connected to the STM32 I/O PC13, wich means Port B pin 13.

Useful links :
[HAL Library](https://www.st.com/resource/en/user_manual/um1725-description-of-stm32f4-hal-and-lowlayer-drivers-stmicroelectronics.pdf) (Section 31.2 for GPIO)
[Informations about pin]()
[Information about PWM]()

## Question 1 : Turn on the User LED

The goal of this question is to create a code that will turn on the User LED.
To achieve that you will create a function in the main file that will turn on the User LED and then call this funtion in the main function, in the infinite loop.

## Question 2 : Blink the User LED

Create a function in the main file that will Toggle the User LED.
Then call this funtion every seconde in the main function, in the infinite loop.

## Question 3 : Use button to turn on the LED

Create a function that will read the value of the button.
If the button is pressed so turn on LED, if the button is not pressed so turn off the LED.
Then call this function in the main function.

## Question 4 : Use button with interruption mode

Same goal than for the previous question but now when the button is pressed it will create an interruption so we don't need to permanently check if the button is pressed on the infinite loop.

For that we have several step to do.

As the button is pressed, the voltage of the pin will go to zero, so we need to detect a falling edge on the value of the voltage.
But we also need to know when the button is released to turn off the LEDs (rising edge).
The mode GPIO_MODE_IT_RISING_FALLING is what we need to achieve that.
When an interruption is triggered, the MCU will execute a handler, which is called HAL_GPIO_EXTI_IRQHandler : 

"""cpp
void HAL_GPIO_EXTI_IRQHandler(uint16_t GPIO_Pin)
{
  /* EXTI line interrupt detected */
  if(__HAL_GPIO_EXTI_GET_IT(GPIO_Pin) != RESET)
  {
    __HAL_GPIO_EXTI_CLEAR_IT(GPIO_Pin);
    HAL_GPIO_EXTI_Callback(GPIO_Pin);
  }
"""

This code will check that the interrupt has been triggered, clear the flag to make sure that the interrupt is not triggered continuously and then call our callback.
This callback can be user defined and we will redefine it in the main file.
In this callback, you will need to check the pin's value (SET or RESET) to work out which edge was triggered (falling or rising).
Write the callback that takes an uint16 as a parameter (the pin that triggered the interrupt) and makes the LEDs react accordingly.

## Question 5 : Turn on LED with PWM

## Question 6 : LED Dimmer
