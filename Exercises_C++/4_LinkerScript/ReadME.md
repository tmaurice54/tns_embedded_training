# Exercise on Linker Script

In this exercise you will learn what is a linker script and how to use it.  
A linker script is a file, here with the .ld extension, that tells the linker which sections to include in the output file, as well as which order to put them in, what type of file is to be produced, and what is to be the address of the first instruction.  

The linker scripts for STM32 project usually contains 2 memories definition: FLASH and RAM.  
Those memories definition are composed of the start address and the size, for example `FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 500K`.  
They will contain sections which will determinate where are the data/functions in the memory.  

Useful information:

The command `readelf` can give a lot of information about .elf files, such as information about the different sections in the file.  

The command `objcopy` allows the user to copy and modify a file, as changing its extension or changing its data in it ...

Useful links:  

[Information about linker script](https://users.informatik.haw-hamburg.de/~krabat/FH-Labor/gnupro/5_GNUPro_Utilities/c_Using_LD/ldLinker_scripts.html)

## Question 1: Find the signature [Renode and Real board]

First of all, you can build the exercise as usual.  
However, the executable .elf that you will create contain a new section.  
Using the debugger and the information given above try to find this new section in the memory at the address: 0x0801E900 and find the message I wrote.  
You can also create a program that will read at this address and print it in a serial terminal.  

## Question 2: Change the signature 1 [Renode and Real board]

Now that you have find the signature, try to modify it in the .ld file.  
You must find the right section and modify the content.  
Then you can compile and use the debugger again to see that the signature is now modified.

You can also change the signature and use the RobotFramework test to test if the changes operate.  

## Question 3: Change the signature 2 [Renode and Real board]

The way to change the signature just before works but is not interesting.  
The linker script should be used just to allocate memory and give the starting address for the signature but not to write it.  
To sign your executable, you will use the command `arm-none-eabi-objcopy`.  
Create a new file and write in it the signature you want to write.  
Then, use the command to append the signature inside the .elf file.  
You can check the result the same way as before.  
