# Exercise on ADC

ADC stands for Analog to Digital Converter, it is a peripheral used to convert an electrical signal (analog voltage) to a binary value.
In this exercise we will work with a potentiometer and read his value with the ADC.

A potentiometer is a three-terminal resistor with a sliding or rotating contact that forms an adjustable voltage divider (Wikipedia).

So potentiometer is a classic resistor with a third terminal which will be connected to the ADC, and depend on the position of the potentiometer, the read value change.

Informations:

- ADC1 is on PA0

Links:

[Pin map](https://os.mbed.com/platforms/ST-Nucleo-F401RE/)
[ADC with STM32](https://www.digikey.com/en/maker/projects/getting-started-with-stm32-working-with-adc-and-dma/f5009db3a3ed4370acaf545a3370c30c)
[HAL Library](https://www.st.com/resource/en/user_manual/um1725-description-of-stm32f4-hal-and-lowlayer-drivers-stmicroelectronics.pdf)

## Question 1: Read value from ADC

For the connection with the board you can connect the left terminal to the `GND`, the middle one to PA0 and the right one to `3V3`.
Once you have connected your potentiometer to the board, you will have to read his value.
You can use the following code to read value from `ADC1` and save it in a variable:

```C
HAL_ADC_Start(&hadc1);
HAL_ADC_PollForConversion(&hadc1,HAL_MAX_DELAY);
uint16_t myValue = HAL_ADC_GetValue(&hadc1);
```

You can then use something like the following to print the value in a terminal such as putty:

```C
sprintf(myMessage, "%hu\r\n", myValue);
HAL_UART_Transmit(&huart2, (uint8_t*)myMessage, strlen(msg), HAL_MAX_DELAY);
```

You should see the value change between 0 and 4096.

## Question 2: Change the brightness of the Led

Create a code that will change the brightness of the user led (or another one) depends on the position of the potentiometer.
You will have to decomment some code in the `main` and in the `MX_TIM2_Init` functions. (Leave commented if not used)  
And the period is already set to 4096, so you can directly use the value from `ADC1` to the `TIM2->CRR1` register.
You don't need to modify the value from the `ADC1`.  
Don't forget to start the `PWM Timer2` on the channel 1 for the User Led.

## Question 3: Control the servomotor

Create a code that will control the position of the servo motor with the potentiometer.
You will have to decomment some code in the `main` and in the `MX_TIM2_Init` functions. (Leave commented if not used)  
You can then use the `Timer 2` for PWM generation.
Don't forget that the value from `ADC1` is in the interval `0..4096` and that the value for the servomotor needs to be inside the interval `0..150`.  
When you start your `PWM Timer2` don't forget to use the channel on which the servomotor is connected.
