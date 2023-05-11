# Repository of the Embedded Training

Welcome to the Embedded Training.
In this repository you will find exercises to learn and discover embedded development.
This training has been created for anyone, even if you didn't do embedded development before.
You can do the exercises following their number so the difficulty increase exercise by exercise.
But you can also select an exercise depend on the subject you want to train.

Note that this training has been prepared to be done with a STM32F401RE board and not for another board.

## Exercises C++ folder

Contains all the main exercises of this training.
Each exercise is made to learn one or multiple specific subjects on embedded development.
The difficulty increases exercise by exercise but also question by question through the same exercise.

Note that the majority part of the support code used for those exercises were created with STM32CubeIDE.
If you want to create your own projects, I recommend you install this IDE.
It will help you to create every file you need (such as drivers, linkerScript, HAL library ...).
You can also check all the cmake file in this training to look at the compilation options and parameters.

## Exercises Rust folder

This folder contains some exercises like the C++ exercises but for Rust project.
Those exercises are an initiation to the rust embedded projects.

## Doc folder

In this folder you will find every information you need to do this training such as the toolchain to install,
Renode setup and tutorial, how flash and debug the STM32 board, etc.

## RenodeConfig folder

In this folder you will find the necessary file to use Renode on this training.
You can find the configuration file `board.repl` to start the machine on Renode.

## RobotFrameWorkTests folder

All the renode/robotframework tests are in this folder.
They are separate between exercises and for each exercise there is tests for each questions.
