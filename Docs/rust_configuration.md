# How to configure Rust

## Install Rust

To install Rust click on [this link](https://www.rust-lang.org/tools/install)
If you are on Windows you will have to download an installer and execute it.  
And if you are on Linux (or subsystem like wsl) you will have to excute a single command in the terminal.

## Install toolchain for the exercises

First make sure you have the newest version of rust :

`rustup update`

For bandwidth and disk usage concerns, the default installation only supports native compilation.
To add cross compilation support for the Arm Cortex-M architectures, you’ll have to install them separately.

For Cortex-M0, M0+, and M1 (ARMv6-M architecture):

`rustup target add thumbv6m-none-eabi`

For Cortex-M3 (ARMv7-M architecture):

`rustup target add thumbv7m-none-eabi`

For Cortex-M4 and M7 without hardware floating point (ARMv7E-M architecture):

`rustup target add thumbv7em-none-eabi`

And finally, for Cortex-M4F and M7F with hardware floating point (ARMv7E-M architecture) (this one for the STM32F401RE):

`rustup target add thumbv7em-none-eabihf`

Then you can install all the modules used to code and flash embedded programms :

`cargo install cargo-binutils cargo-embed cargo-flash cargo-expand`

`rustup component add llvm-tools-preview`

## Compile and flash

First open a terminale (Linux  or Windows) and go in your project directory.

To compile your project and create a binary in release mode :

`cargo build --release`

To flash your programm with `cargo-flash` :

`cargo flash --chip STM32F401RETx --release`

Or using `cargo-embed` :

`cargo embed --target thumbv7em-none-eabihf`

To flash an example (files in examples directory) :

`cargo flash --chip STM32F401RETx --example fileName`

You may to change the parameter for `--chip` or `--target` depend of the board you are using.
But note that all the config files in the exercises directories are wrote to work with the STM32F401RE board.
You may need to change some parameters if you need to work with an another board.
