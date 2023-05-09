# Exercice on sensor

In this exercise you will discover how to use SPI sensor with STM32 board using rust language.
For this exercise you will need Renode because we don't have access to a SPI sensor.  
But the board we can simulate on renode has the temperature sensor TI LM74 connected on the SPI2.

This temperature sensor communicates with the SPI (Serial Peripheral Interface) protocol.  
With SPI communication, the circuits communicate in a master-slave scheme, where the master controls the communication.  
The master is connected to his slaves with 4 ports :

- CLK to share the clock with the slaves
- MISO (Master Input, Slave Output) to send data to the slaves
- MOSi (Master Ouput, Slave Input) to receive data from the slaves
- SS (Slave Select) used to select the slave the master want to communicate with

CLK, MOSI and MISO are the same pin on the master for all the slaves.
That's why there is the SS port wich is different for each slave.  
The value of SS is 1 by default, and when we want to start a communication with a specific slave, we change the value to 0 and then start the communication.  
More information about the SPI Protocol [here](https://www.circuitbasics.com/basics-of-the-spi-communication-protocol/).

Useful informations :  

In this exercise we will use the HAL library and the SPI2 of the STM32 board.
Pin configuration on the STM32F401RE board for the SPI2

- SS : PB9 or PB12  
- MISO : PC2  
- MOSI : PC3  
- Clock : PB10  

The main.cpp file already has a handler for the SPI2 and the initilisation is already done.

Useful links :  
[Crate stm32f4xx_hal](https://docs.rs/stm32f4xx-hal/latest/stm32f4xx_hal/)  
[Information about pin](https://os.mbed.com/platforms/ST-Nucleo-F401RE/)  
[Rust embedded doc](https://docs.rust-embedded.org/book/intro/index.html/)  
[TI LM74 Datasheet](https://pdf1.alldatasheet.net/datasheet-pdf/view/9026/NSC/LM74.html)  

## Question 1 : Get data from SPI [Renode]

Our objective is to read the temperature of a sensor connected via SPI.  
After looking at the documentation about the LM74 sensor and on how to use SPI with STM32 board,
create a function that will read the temperature and put the value in a buffer.  
Don't forget to change the value of SS to 0 send/receive data, and after the communication is done to change again the value to 1.  
Also the LM74 sensor send data that you have to process to get the temperature value. (Look at the datasheet)  

## Question 2 : Interface for the sensor [Renode]

Now you have understand how to read values from SPI Sensor.
But this kind of code isn't practical and developper usually create an interface to simplify their code and the futur utilisation of the sensor.  
Create an interface for this sensor wich will contain a function get_temp() to read the temperature and use it in your main function.

A possibilitie is to create a `struct` and then use `impl` to implemente function for this struct.
