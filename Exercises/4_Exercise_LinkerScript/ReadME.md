# Exercise on Linker Script

In this exercise you will learn what is a linker script and how to use it.  
A linker script is file, here with the .ld exentension, that tells the linker which sections to include in the output file, as well as which order to put them in, what type of file is to be produced, and what is to be the address of the first instruction.  

The linker scripts for STM32 project usually contains 2 memories definition : FLASH and RAM.  
Those memories definition designate the start address and the size, for example  ```FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 500K```.  
They will contains sections which will determinate where are the data in the memory.  

Useful information :

The command ```readelf``` can give a lot of information about .elf files, such as information about the different sections in the file.  

The command ```objcopy``` allow the user to copy and modify a file, as changing its extension or changing its data in it ...

Useful links :  

[Information about linker script](https://users.informatik.haw-hamburg.de/~krabat/FH-Labor/gnupro/5_GNUPro_Utilities/c_Using_LD/ldLinker_scripts.html)

## Question 1 : Find the signature [Renode and Real board]

First of all you can build the exercise as usual.  
But the executable you will create contain a new section.  
Using the debugger and the informations given above try to find this new section in the memory and find the message I wrote.  

## Question 2 : Change the signature 1 [Renode and Real board]

Now that you have find the signature I have done, try to modifie it in the .ld file.  
You have to find the right section and modifie the content.  
Then you can use the debugger again and see that the signature is now modified.

You can also change the signature and use the RobotFramework test.  

## Question 3 : Change the signature 2 [Renode and Real board]

This way to change the signature works but is not interesting.  
The linker script should be used just to allocate memory for the signature but not to write it.  
To sign your executable you will use the command ```arm-none-eabi-objcopy```.  
Create a new file and write in it the signeture you want to write.
Then use this command to write this content in the memory allocated for the signature.  
