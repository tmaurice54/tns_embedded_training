# Exercise RTIC

In this exercise we will use the RTIC (Real-Time Interrupt-driven Concurrency) framework.
This is a concurrency framework for building real-time systems.
But the difference with an usual RTOS system is that RTIC uses the NVIC in Cortex-M MCUs to perform scheduling.
When creating tasks, we have to indicates on which interrupt it will be triggered.

All you need to know for this exercise is in the RTIC documentation and in the previous exercises.
But if you struggle to find the solution you can look at the `examples` directory which contains examples of solutions for the different questions.

Useful links:  
[RTIC Book](https://rtic.rs/1/book/en/)  
[RTIC Crates Doc](https://docs.rs/cortex-m-rtic/latest/rtic/)  

## Question 1 : Create an interrupt on button

By looking at the documentation and with the template of RTIC programm given (main.rs),
create a task that will toggle the led when you press the button.
The initialization of the device/led/button will be similary the same as for the interrupt question in the GPIO exercise.

## Question 2 : Create an interrupt on Timer

Now create a task that will start on a Timer interrupt.
Each time the Timer ends, the task starts again, and toggles the led.
Ths will create a task that will blink the led, controlled by Timer interrupt.

## Question 3 : Control the blink delay using the button

Now use what you learned in the two previous question to create a task that will blink the led depending on a global delay.
And create a task triggered by the button interrupt that will change the delay of the blink.
