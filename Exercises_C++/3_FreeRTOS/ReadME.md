# Exercise on FreeRTOS

In this exercise you will learn how to use the Real Time OS FreeRTOS.  

Real Time OS are for real-time computing applications that processes data and events that have critically defined time constraints.
Real-time operating systems are event-driven and pre-emptive, meaning the OS can monitor the relevant priority of competing tasks, and make changes to the task priority.
Event-driven systems switch between tasks based on their priorities, while time-sharing systems switch the task based on clock interrupts. (Wikipedia)  

So, a Real Time OS allows the user to create tasks that can be controlled with events/interruption/semaphores, or tasks that can work simultaneously if they have the same priority.  
FreeRTOS is the leader in the RTOS area and is commonly used in industry.  

To use FreeRTOS in this exercise an include `#include "FreeRTOS.h"` is made in the file `main.h`.
Also to create tasks and use semaphores the includes `#include "task.h"` and `#include "semphr.h"` are already made in the main file.
If you want to use other functionalities of FreeRTOS you may need to include other headers.

Useful links:  
[FreeRTOS API Reference](https://www.freertos.org/a00106.html)
[FreeRTOS Manual](https://www.freertos.org/Documentation/FreeRTOS_Reference_Manual_V9.0.0.pdf)  
[FreeRTOS Developer docs](https://www.freertos.org/features.html)

## Question 1: Create your first task [Renode and Real board]

Your first question will be to create a task that will blink the user LED.

To achive that you must create a function named `blinkTask` as follow:  

```cpp
void blinkTask(void *pvParameters)
{
    while(1)
    {
        //code to blink the LED
    }
    vTaskDelete( NULL );
}
```

If needed you can pass arguments to the task, but here we don't need to.  
After writing your task handler, you will need to create the task in the main function:

```cpp
xTaskCreate(blinkTask,"Blink task", 128, NULL, 1, NULL);
```

More information about the function [here](https://www.freertos.org/a00125.html)

This task creation should be done before the function call `vTaskStartScheduler` which will give the control to the kernel and start the tasks.

## Question 2: Control tasks with semaphore [Renode and Real board]

Now that you have understood how a task works, create 2 tasks to blink the LED.  
One that will turn on the LED and on that will turn off the LED.
To controls those tasks, you will need a semaphore. (With the function `xSemaphoreGive` and `xSemaphoreTake`)

The handle is already created for the semaphore.
You just need to initiate it in the main function with the `xSemaphoreCreateBinary()`  function.  
If you need information about what a semaphore is and how to use it, look at this link:
[Introduction to semaphore with FreeRTOS by digikey](https://www.digikey.com/en/maker/projects/introduction-to-rtos-solution-to-part-7-freertos-semaphore-example/51aa8660524c4daba38cba7c2f5baba7)

## Question 3 : Control tasks with interruption [Renode and Real board]

Tasks can also be controlled with interruptions as we have seen in the GPIO exercise.  
Create a function that will toggle the LED each time you press the user button.  
The user button will create an interrupt and release a semaphore to unblock the task.

Note that to realease a semaphore in an interruption callback you need the function : `xSemaphoreGiveFromISR(myBinaryHandle, pdFALSE)`

## Question 4: Filter value of a SPI temperature sensor [Renode]

Prerequisite: Exercises on SPI

For this exercise you will reuse the interface you have coded for the SPI sensor.
In this interface you will add a new function `getFilteredTemp()`.
The idea is to create a task that reads the temp value every 200ms and puts the value in a array of 10 temperature type FIFO.
And the `getFilteredTemp()` eliminates the smallest and greatest values and return the average of the 8 temperature values left.  

When working with sensors, we usually filter the values to work with less noisy data.

## Question 5: Filter value of a I2C temperature sensor [Real board]

Prerequisite: Exercises on I2C

For this exercise you will reuse the interface you have coded for the I2C sensor.
In this interface you will add a new function `getFilteredTemp()`.
The idea is to create a task that reads the temp value every 200ms and puts the value in a array of 10 temperature type FIFO.
And the `getFilteredTemp()` eliminates the smallest and greatest values and return the average of the 8 temperature values left.  

When working with sensors, we usually filter the values to work with less noisy data.
