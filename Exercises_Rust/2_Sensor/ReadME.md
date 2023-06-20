# Exercise on sensor

In this exercise you will discover how to use a SPI ans I2C sensor with STM32 board.
The SPI part will be on Renode and the I2C part will be done with the real board and a sensor.

## SPI information

The sensor LM74 that can be simulate on renode communicates with the SPI (Serial Peripheral Interface) protocol.  
With SPI communication, the circuits communicate in a master-slave scheme, where the master controls the communication.  
The master is connected to his slaves with 4 ports:

- CLK to share the clock with the slaves
- MISO (Master Input, Slave Output) to send data to the slaves
- MOSI (Master Output, Slave Input) to receive data from the slaves
- SS (Slave Select) used to select the slave the master wants to communicate with

CLK, MOSI and MISO are the same pin on the master for all the slaves.
That's why there is the SS port which is different for each slave.  
The value of SS is 1 by default, and when we want to start a communication with a specific slave, we change the value to 0 and then start the communication.  
More information about the SPI Protocol [here](https://www.circuitbasics.com/basics-of-the-spi-communication-protocol/).

Information:  

In this exercise we will use the HAL library and the SPI2 of the STM32 board.
Pin configuration on the STM32F401RE board for the SPI2:

- SS: PB9 or PB12  
- MISO: PC2  
- MOSI: PC3  
- Clock: PB10  

Useful links:  
[Crate stm32f4xx_hal](https://docs.rs/stm32f4xx-hal/latest/stm32f4xx_hal/)  
[Information about pin](https://os.mbed.com/platforms/ST-Nucleo-F401RE/)  
[Rust embedded doc](https://docs.rust-embedded.org/book/intro/index.html/)  
[TI LM74 Datasheet](https://pdf1.alldatasheet.net/datasheet-pdf/view/9026/NSC/LM74.html)  

## I2C information

The sensor TC74 communicates with I2C communication.
This protocol is a master/slave communication using 2 ports:

- SCLK to share the clock to the slaves
- SDA to share the data

These pins on the master can be used for several slaves.
To differentiate the slaves, each slaves has an address.
This address is used when data are send to commuincate with a specific slave.

Informations :

In this exercise we will use the HAL library and the I2C3 of the STM32 board.
Pin configuration on the STM32F401RE board for the I2C3:

- SCLK: PA8
- SDA: PC9

Useful links:  
[Crate stm32f4xx_hal](https://docs.rs/stm32f4xx-hal/latest/stm32f4xx_hal/)  
[Information about pin](https://os.mbed.com/platforms/ST-Nucleo-F401RE/)  
[Rust embedded doc](https://docs.rust-embedded.org/book/intro/index.html/)
[TC74 Datasheet](https://www.alldatasheet.com/datasheet-pdf/pdf/75085/MICROCHIP/TC74.html)  

## Question 1: Get data from SPI [Renode]

Our objective is to read the temperature of a sensor connected via SPI.  
After looking at the documentation about the LM74 sensor and on how to use SPI with STM32 board,
create a function that will read the temperature and put the value in a buffer.  
Don't forget to change the value of SS to 0 send/receive data, and after the communication is done to change again the value to 1.  
The LM74 sensor sends data that you have to process to get the temperature value. (Look at the datasheet)  

## Question 2: Interface for the sensor [Renode]

Now you have understood how to read values from a SPI Sensor.
But this kind of code isn't practical, and developers usually create an interface to simplify their code and the future utilisation of the sensor.  
Create an interface for this sensor which will contain a function get_temp() to read the temperature and use it in your main function.

A possibility is to create a `struct` and then use `impl` to implement function for this struct.

## Question 3: Get data from I2C [Real board]

Our objective is to read the temperature of a sensor connected via I2C.  
After looking at the documentation about the TC74 sensor and on how to use I2C with STM32 board,
create a function that will read the temperature and put the value in a buffer.  

While connecting the sensor to the board, don't forget to also connected the pin SDA and SCL of the sensor to GND or VCC with a resistor.

## Question 4: Interface for the I2C sensor [Real board]

Now you have understood how to read values from a I2C Sensor.
But this kind of code isn't practical, and developers usually create an interface to simplify their code and the future utilisation of the sensor.  
Create an interface for this sensor which will contain a function get_temp() to read the temperature and use it in your main function.

A possibility is to create a `struct` and then use `impl` to implement function for this struct.

## Question 5: Display the temperature on 7 Segment display [Real board]

You know how to read the temperature from a temperature sensor and you know how to display digits from the exercise on GPIO.
Let's create a thermometer.
You can use 2 7-segment display and you can also display negativ temperature if you use the `dp` of the display.

For the pins you can use the pins PA0,PA1,PA4,PA5,PA6,PA7 and PA9 for one digit and PB0,PB1,PB4,PB5,PB6 and PB7.
Don't forget to initate these pins.
