# How to build the exercises

## Installing the ARM toolchain

Assuming you run a WSL machine

### From the package manager

```bash
sudo apt-get install gcc-arm-none-eabi
```

### From the sources

Download the ARM toolchain:

``` bash
wget -O gcc-arm-none-eabi.tar.xz https://developer.arm.com/-/media/Files/downloads/gnu/12.2.mpacbti-bet1/binrel/arm-gnu-toolchain-12.2.mpacbti-bet1-x86_64-arm-none-eabi.tar.xz
```

You can then install it in the folder of your choice and add the binaries to your path:

``` bash
tar xf ./gcc-arm-none-eabi.tar.xz --strip-components=1 -C <FOLDER_PATH_HERE>
echo 'export PATH=$PATH:<FOLDER_PATH_HERE>/bin' >> ~/.bashrc
```

### Testing the toolchain

```bash
arm-none-eabi-gcc --version
```

### Package need for check and code formatage

```bash
sudo apt-get install cppcheck
sudo apt-get install clang-format
```

## Building the exercise

Prerequisites: (sudo apt install)

- Cmake  
- Ninja

Open a wsl terminal and go to the exercise folder you want to build.
Then execute:

```bash
./build.sh
```

which will create a build folder and put the executable in it.

If you just want to compile again after you have done some modifications, you can go in the build folder and execute:

```bash
ninja
```
