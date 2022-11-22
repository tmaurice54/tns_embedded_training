cmake_minimum_required(VERSION 3.15)
# to avoid vs code error as compiler can't compile a simple program
# https://stackoverflow.com/a/53635241
set(CMAKE_TRY_COMPILE_TARGET_TYPE "STATIC_LIBRARY")
project(nucleo-blinky)

enable_language(CXX C ASM)

# set(CMAKE_C_STANDARD C99)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

set(GENERATED_FILES 
    Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_tim.c
    Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_tim_ex.c
    Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_uart.c
    Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_rcc.c
    Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_rcc_ex.c
    Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_flash.c
    Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_flash_ex.c
    Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_flash_ramfunc.c
    Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_gpio.c
    Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_dma_ex.c
    Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_dma.c
    Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_pwr.c
    Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_pwr_ex.c
    Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_cortex.c
    Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal.c
    Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_exti.c
    Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_pcd.c
    Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_pcd_ex.c
    Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_ll_usb.c


    Core/Inc/main.h
    Core/Inc/stm32f4xx_hal_conf.h
    Core/Inc/stm32f4xx_it.h


    Core/Src/main.cpp
    Core/Src/syscalls.c
    Core/Src/sysmem.c
    Core/Src/stm32f4xx_hal_msp.c
    Core/Src/stm32f4xx_it.c
    Core/Src/system_stm32f4xx.c
    Core/Startup/startup_stm32f446zetx.s
    
)

set(EXECUTABLE ${PROJECT_NAME}.out)

add_executable(${EXECUTABLE} ${GENERATED_FILES})
target_compile_definitions(${EXECUTABLE} PRIVATE
    -DUSE_HAL_DRIVER
    -DSTM32F446xx
)

target_include_directories(${EXECUTABLE} PRIVATE
    Core/Inc
    Drivers/CMSIS/Device/ST/STM32F4xx/Include
    Drivers/CMSIS/Include
    Drivers/STM32F4xx_HAL_Driver/Inc
)

target_compile_options(${EXECUTABLE} PRIVATE
    -mcpu=cortex-m4
    
    -ffunction-sections
    -fdata-sections 
    -Wall 
    -fstack-usage 
    # -MMD 
    # -MP 
    -mfpu=fpv4-sp-d16 
    -mfloat-abi=hard 
    -mthumb
)

target_link_options(${EXECUTABLE} PRIVATE
        -T${CMAKE_SOURCE_DIR}/STM32F446ZETX_FLASH.ld
        -mcpu=cortex-m4
        -mthumb
        -mfpu=fpv4-sp-d16
        -mfloat-abi=hard
        -specs=nano.specs
        -lc
        -lm
        -Wl,-Map=${PROJECT_NAME}.map,--cref
        -Wl,--gc-sections
        )

# Print executable size
add_custom_command(TARGET ${EXECUTABLE}
        POST_BUILD
        COMMAND arm-none-eabi-size ${EXECUTABLE})

# Create hex file - Optional
# add_custom_command(TARGET ${EXECUTABLE}
#         POST_BUILD
#         COMMAND arm-none-eabi-objcopy -O ihex ${EXECUTABLE} ${PROJECT_NAME}.hex
#         COMMAND arm-none-eabi-objcopy -O binary ${EXECUTABLE} ${PROJECT_NAME}.bin)
