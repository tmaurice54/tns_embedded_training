# Exercise RTIC

In this exercise we will use the RTIC (Real-Time Interrupt-driven Concurrency) framework.
This is a concurrency framework for building real-time systems.

Useful links:  
[RTIC Book](https://rtic.rs/1/book/en/)  
[RTIC Crates Doc](https://docs.rs/cortex-m-rtic/latest/rtic/)  

## Question 1 : Create an interrupt on button

Create a task that starts on butoon interrupt.
Each time you press the button, the led is toggle.

## Question 2 : Create an interrupt on Timer

Now create a task that will start on a Timer interrupt.
Each time the Timer ends, the task start again, and in this task, just toggle the led.
Ths will create a task that will blink the led, controlled by Timer interrupt.

## Question 3 : Control the blink delay using the button

Now use what you learned in the two previous question to create a task that will blink the led depend on a global delay.
And create a task triggered by the button interrupt that will change the delay of the blink.
