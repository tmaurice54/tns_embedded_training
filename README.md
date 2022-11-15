# Training for embedded systems with Renode

To install the compiler toolchain: 
```
sudo bash setup.sh
```


To create the build files:

```
mkdir cmake-build-debug
cmake -G "Ninja" -DCMAKE_TOOLCHAIN_FILE=../arm-none-eabi-gcc.cmake -DCMAKE_BUILD_TYPE=Debug ..
ninja
```
