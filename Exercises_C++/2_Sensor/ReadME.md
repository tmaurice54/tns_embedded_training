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

Useful information:  

In this exercise we will use the HAL library and the SPI2 of the STM32 board.
Pin configuration on the STM32F401RE board for the SPI2:

- SS: PB9 or PB12  
- MISO: PC2  
- MOSI: PC3  
- Clock: PB10  

The main.cpp file already has a handler for the SPI2 and the initialisation is already done.

Useful links:  
[TI LM74 Datasheet](https://pdf1.alldatasheet.net/datasheet-pdf/view/9026/NSC/LM74.html)  
[How to use SPI with STM32 boards](https://www.digikey.com/en/maker/projects/getting-started-with-stm32-how-to-use-spi/09eab3dfe74c4d0391aaaa99b0a8ee17)  
[HAL Library](https://www.st.com/resource/en/user_manual/um1725-description-of-stm32f4-hal-and-lowlayer-drivers-stmicroelectronics.pdf)

## I2C information

The sensor TC74 communicates with I2C communication.
This protocol is a master/slave communication using 2 ports:

- SCLK to share the clock to the slaves
- SDA to share the data

These pins on the master can be used for several slaves.
To differentiate the slaves, each slaves has an address.
This address is used when data are send to commuincate with a specific slave.

Useful informations :

In this exercise we will use the HAL library and the I2C3 of the STM32 board.
Pin configuration on the STM32F401RE board for the I2C3:

- SCLK: PA8
- SDA: PC9

The main.cpp file already has a handler for the I2C3 and the initialisation is already done.

Useful links:  
[TC74 Datasheet](https://www.alldatasheet.com/datasheet-pdf/pdf/75085/MICROCHIP/TC74.html)  
[HAL Library](https://www.st.com/resource/en/user_manual/um1725-description-of-stm32f4-hal-and-lowlayer-drivers-stmicroelectronics.pdf)
[I2C project example](https://www.digikey.be/en/maker/projects/getting-started-with-stm32-i2c-example/ba8c2bfef2024654b5dd10012425fa23)  

## Question 1: Get data from SPI [Renode]

Our objective is to read the temperature of a sensor connected via SPI.  
After looking at the documentation about the LM74 sensor and on how to use SPI with STM32 board,
create a function that will read the temperature and put the value in a buffer.  
Don't forget to change the value of SS to 0 send/receive data, and after the communication is done to change again the value to 1.  
The LM74 sensor sends data that you have to process to get the temperature value. (Look at the datasheet)  

To test your code with the Robot Framework test, turn on the user LED if the value of the Temperature is 50, and turn the LED off otherwise.

## Question 2: Interface for the sensor [Renode]

Now you have understood how to read values from SPI Sensor.
But this kind of code isn't practical, and developers usually create an interface to simplify their code and the future utilisation of the sensor.
In a new file, create an interface for this sensor which will contain a function readTemp() to read the temperature.
After that you will create an instance in your main file and try to read the temperature value.  

## Question 3: Get data from I2C [Real board]

Our objective is to read the temperature of a sensor connected via I2C.  
After looking at the documentation about the TC74 sensor and on how to use I2C with STM32 board,
create a function that will read the temperature and put the value in a buffer.  

## Question 4: Interface the sensor [Real board]

Now you have understood how to read values from I2C Sensor.
But this kind of code isn't practical, and developers usually create an interface to simplify their code and the future utilisation of the sensor.
In a new file, create an interface for this sensor which will contain a function readTemp() to read the temperature.
After that you will create an instance in your main file and try to read the temperature value.  

## Question 5: Read/Write value to an EEPROM [Real board]

The EEPROM (Electrically Erasable Programmable Read-Only Memory) may also used I2C protocol.
In This exercise you will have to write and read data from/to an EEPROM.

For that you will use what you learned before on the I2C protocol.
To read and write data you may want to use the function `HAL_I2C_Mem_Write` and `HAL_I2C_Mem_Read` of the STM32 HAL library.
With these functions you can choose the address of the data inside the EEPROM you want to work with.

[Example of project with EEPROM](https://controllerstech.com/eeprom-and-stm32/)
[EERPOM Datasheet](https://docs.rs-online.com/7608/0900766b813214df.pdf)
