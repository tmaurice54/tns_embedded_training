# Training for embedded systems with Renode
 
## Installing the ARM toolchain
```
sudo bash setup.sh
```

## Building the project
```
mkdir cmake-build-debug
cmake -G "Ninja" -DCMAKE_TOOLCHAIN_FILE=../arm-none-eabi-gcc.cmake -DCMAKE_BUILD_TYPE=Debug ..
ninja
```
