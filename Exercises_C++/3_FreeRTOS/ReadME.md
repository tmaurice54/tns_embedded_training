# Exercise on FreeRTOS

In this exercise you will learn how to use the Real Time OS FreeRTOS.  

Real Time OS are for real-time computing applications that processes data and events that have critically defined time constraints.
Real-time operating systems are event-driven and pre-emptive, meaning the OS can monitor the relevant priority of competing tasks, and make changes to the task priority.
Event-driven systems switch between tasks based on their priorities, while time-sharing systems switch the task based on clock interrupts. (Wikipedia)  

So, a Real Time OS allows the user to create tasks that can be controlled with events/interruption/semaphores, or tasks that can work simultaneously if they have the same priority.  
FreeRTOS is the leader in the RTOS area and is commonly used in industry.  

But here we won't use directly FreeRTOS:  
It’s important to understand how STM32CubeIDE has bundled FreeRTOS.
While FreeRTOS is an underlying software framework that allows for switching tasks, scheduling, etc., we won’t be making calls to FreeRTOS directly.
ARM has created the CMSIS-RTOS library, which allows us to make calls to an underlying RTOS, thus improving the portability of code among various ARM processors. (digikey.com)

Useful links:  
[FreeRTOS/CMSIS on STM32](https://www.digikey.com/en/maker/projects/getting-started-with-stm32-introduction-to-freertos/ad275395687e4d85935351e16ec575b1)  
[FreeRTOS Manual](https://www.freertos.org/Documentation/FreeRTOS_Reference_Manual_V9.0.0.pdf)  
[CMSIS functions overview](https://www.keil.com/pack/doc/CMSIS/RTOS/html/functionOverview.html)  
[Semaphore with CMSIS](https://www.keil.com/pack/doc/CMSIS/RTOS/html/group__CMSIS__RTOS__SemaphoreMgmt.html)

## Question 1: Create your first task [Renode and Real board]

Your first question will be to create a task (called Thread in CMSIS but Task by FreeRTOS) that will blink the user LED.
To achieve that the main file contains already the handle and the attributes for the task (line 46).
You can see in `blinkTask_attributes` that the task name is `blinkTask` and that the priority is already set to normal.  

Now you must create a function named `blinkTask` as follow:  

```cpp
void blinkTask()
{
    while(1)
    {
        //code to blink the LED
    }
}
```

If needed you can pass arguments to the task, but here we don't need to.  
After writing your task handler, you will need to create the task in the main function:
```cpp
blinkTaskHandle = osThreadNew(blinkTask, NULL, &blinkTask_attributes);
```

This task creation should be done before the function call `osKernelStart` which will give the control to the kernel and start the tasks.

## Question 2: Control tasks with semaphore [Renode and Real board]

Now that you have understood how a task works, create 2 tasks to blink the LED.  
One that will turn on the LED and on that will turn off the LED.
To controls those tasks, you will need a semaphore. (With the function `osSemaphoreRelease` and `osSemaphoreAcquire`)

The handles and attributes are already created for the semaphore.
You just need to create it in the main function with the `osSemaphoreNew()`  function.  
If you need information about what a semaphore is and how to use it, look at this link:
[Semaphore with CMSIS](https://www.keil.com/pack/doc/CMSIS/RTOS/html/group__CMSIS__RTOS__SemaphoreMgmt.html)

## Question 3 : Control tasks with interruption [Renode and Real board]

Tasks can also be controlled with interruptions as we have seen in the GPIO exercise.  
Create a function that will toggle the LED each time you press the user button.  
The user button will create an interrupt and release a semaphore to unblock the task.

## Question 4: Filter value of a temperature sensor [Renode]

Prerequisite: Exercises on SPI

For this exercise you will reuse the interface you have coded for the SPI sensor.
In this interface you will add a new function `getFilteredTemp()`.
The idea is to create a task that reads the temp value every 200ms and puts the value in a array of 10 temperature type FIFO.
And the `getFilteredTemp()` eliminates the smallest and greatest values and return the average of the 8 temperature values left.  

When working with sensors, we usually filter the values to work with less noisy data.
