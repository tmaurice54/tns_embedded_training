# Exercise Hello World

In this exercise 0 you will discover the environment used in this training (for the C/C++ part).

All the directory contains:

- `Core` directory which contains 4 directory:
  - `Inc` contains the includes
  - `Src` contains the source files with the `main` file, which is the file you will modify for the exercise.
  - `Startup` and `Stmfiles` contains other necessary files to compile the project
- a script `build.sh` that will create a `build` folder and compiles/creates the excutable inside. You can use it to compile the exercise
- `CMakeLists.txt` file used for the compilation
- `STM32F401RETX_FLASH.ld` file which is the linker script use for the compilation

## Question 1: Open the main file

First of all you can open the main file to familiarise with it.

All the files used in this C/C++ part has been generated with STM32CubeIDE, so you can see that the main file contains already some functions and comment.  
It contains a `SystemClock_Config` to init the clock, `MX_GPIO_Init` to init the pin GPIO pin of the board and `MX_USART2_UART_Init` to init the UART2 that can be used for communication with your computer.  
It contains also a `main` function which first call these init functions and then go in a infinite loop.
This infinite loop is used for the device to never stop and to execute an/multiple action(s) indefinitely.  
For now the `main` function only blink the user led every second.

## Question 2: Compile the exercise

The exercise don't need modification to compile so let's try.  
Just execute these commands:

```bash
./build.sh
cd build
ninja
```

If the compilation went well, you should find your `.elf` executable: `build/0_HelloWorld/Exercise_HelloWorld.elf`.

For more information about the configuration and compilation [here](../../Docs/C%2B%2B_configuration.md)

## Question 3: Flash your code with Ozone [Need Real Board]

Now that you have your executbale ready, you need to flash it inside your board.  
To dot it, you can follow the steps in [this file](../../Docs/use_ozone.md)

You should see the user led blinking.

## Question 4: Flash your code in Renode [Need Renode]

If you don't have the real board, you can simulate it in renode and test your code with it.  
To do it, you can follow the steps for renode [here](../../Docs/use_renode_robotframework.md)  

With the command `sysbus.gpioPortA.LD2 State` you can see the state of the user led at the current time.
If you execute this command every second, you should see the state changing, meaning that the led is blinking.

## Question 5: Unit testing with Renode and RobotFramework [Need Renode]

You can also execute some unit test, depends on the exercise.  
You can look at the [this](../../Docs/use_renode_robotframework.md) doc to look at how install Robotframework and execute the tests.
