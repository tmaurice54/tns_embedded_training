#!/bin/bash
rm -r build

mkdir build && cd build

cmake -G "Ninja" -DCMAKE_TOOLCHAIN_FILE=./arm-none-eabi-gcc.cmake -DCMAKE_BUILD_TYPE=Debug -DEXERCICE=HELLOWORLD ../.. 

ninja