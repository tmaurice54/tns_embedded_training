# Repository to create the Embedded Training

## Base Folder

Contain the intial code without fonctionnalities

## Exercise_LED Folder

### BareMetal

Contains the code to create an interrupt on the userbutton.
Each time the button is pressed/release, the LED1 is toggled.

### FreeRTOS

Contains 2 task which turn on and turn off the LED1 every second.

## Exercise_SPI Folder

### BareMetal

Contains the code to read the temperature on the TI_LM74 sensor.
And a wrapper for this sensor.

### FreeRTOS

No specific code
