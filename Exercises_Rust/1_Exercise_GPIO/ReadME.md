# Exercise on GPIO

This first exercise introduces the usage of GPIOs using Rust language.
You will control a led and a button.
In this exercise you will be using the STM32 HAL library which provides basic APIs to facilitate the utilization of your STM32 board.

Useful informatons :

The board STM32F401RE come with a User Led (named led 2) connected to the STM32 I/O PA5, wich means Port A Pin 5.
It also come with a User button connected to the STM32 I/O PC13, wich means Port B pin 13.

Useful links :  
[Information about pin](https://os.mbed.com/platforms/ST-Nucleo-F401RE/)  

## Question 1 : Turn on the User LED [Renode and Real Board]

The goal of this question is to create a code that will turn on the User LED.  
To achieve that you will create a function in the main file that will turn on the User LED and then call this funtion in the main function, in the infinite loop.

## Question 2 : Blink the User LED [Renode and Real Board]

Create a function in the main file that will Toggle the User LED.  
Then call this funtion every seconde in the main function, in the infinite loop.

## Question 3 : Use button to turn on the LED [Renode and Real Board]

Create a function that will read the value of the button.
If the button is pressed, turn on LED and if the button is not pressed, turn off the LED.
Then call this function in the main function.

(Caution : On renode a button pressed mean 1 when it actually mean 0 on the real board)

## Question 4 : Use button with interruption mode [Renode and Real Board]

## Question 5 : Turn on LED with PWM [Real board]

## Question 6 : LED Dimmer [Real board]
