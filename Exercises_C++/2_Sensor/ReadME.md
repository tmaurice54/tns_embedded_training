# Exercise on sensor

In this exercise you will discover how to use SPI sensor with STM32 board.  
For this exercise you will need renode because we don't have access to a SPI sensor.  
But the board we can simulate on renode has the temperature sensor TI LM74 connected on the SPI2.

This temperature sensor communicates with the SPI (Serial Peripheral Interface) protocol.  
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
Pin configuration on the STM32F401RE board for the SPI2

- SS: PB9 or PB12  
- MISO: PC2  
- MOSI: PC3  
- Clock: PB10  

The main.cpp file already has a handler for the SPI2 and the initialisation is already done.

Useful links:  
[TI LM74 Datasheet](https://pdf1.alldatasheet.net/datasheet-pdf/view/9026/NSC/LM74.html)  
[How to use SPI with STM32 boards](https://www.digikey.com/en/maker/projects/getting-started-with-stm32-how-to-use-spi/09eab3dfe74c4d0391aaaa99b0a8ee17)  
[HAL Library](https://www.st.com/resource/en/user_manual/um1725-description-of-stm32f4-hal-and-lowlayer-drivers-stmicroelectronics.pdf)

## Question 1: Get data from SPI [Renode]

Our objective is to read the temperature of a sensor connected via SPI.  
After looking at the documentation about the LM74 sensor and on how to use SPI with STM32 board,
create a function that will read the temperature and put the value in a buffer.  
Don't forget to change the value of SS to 0 send/receive data, and after the communication is done to change again the value to 1.  
Also, the LM74 sensor send data that you have to process to get the temperature value. (Look at the datasheet)  

To test your code with the Robot Framework test, turn on the user LED if the value of the Temperature is 50, and let the LED off in the other cases.

## Question 2: Interface for the sensor [Renode]

Now you have understood how to read values from SPI Sensor.
But this kind of code isn't practical, and developer usually create an interface to simplify their code and the future utilisation of the sensor.  
In a new file create an interface for this sensor which will contain a function readTemp() to read the temperature.
After that you will create an instance in your main file and try to read the temperature value.  
