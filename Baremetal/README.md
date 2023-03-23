# Baremetal Sample code

## Building the sample

``` bash
mkdir cmake-build-debug && cd cmake-build-debug
cmake -G "Ninja" -DCMAKE_TOOLCHAIN_FILE=../arm-none-eabi-gcc.cmake -DCMAKE_BUILD_TYPE=Debug ..
ninja
```

## Testing the sample

If renode is installed on Windows :

``` powershell
renode-test "<Demo folder location>\test\test.robot"
```

**If you use WSL to compile** : Use "\\wsl`$\distro_name\home\... as a base for the path to the robot file
