# Training for embedded systems with Renode

## Installing Renode
For Windows:  (need admin rights)
- go to [https://github.com/renode/renode/releases](https://github.com/renode/renode/releases) and download the latest version of the .msi installer. 

## Installing the ARM toolchain

Assuming you run a WSL machine

Download the ARM toolchain

``` bash
wget -O gcc-arm-none-eabi.tar.xz https://developer.arm.com/-/media/Files/downloads/gnu/12.2.mpacbti-bet1/binrel/arm-gnu-toolchain-12.2.mpacbti-bet1-x86_64-arm-none-eabi.tar.xz
```

You can then install it in the folder of your choice and add the binaries to your path

``` bash
tar xf ./gcc-arm-none-eabi.tar.xz --strip-components=1 -C <FOLDER_PATH_HERE>
echo "export PATH=$PATH:<FOLDER_PATH_HERE>" >> ~/.bashrc
```

## Building the project

**You need to be in either the Baremetal or FreeTROS folder**. It is better to open one of these folder in vs code directly

``` bash
mkdir cmake-build-debug && cd cmake-build-debug
cmake -G "Ninja" -DCMAKE_TOOLCHAIN_FILE=../arm-none-eabi-gcc.cmake -DCMAKE_BUILD_TYPE=Debug ..
ninja
```
