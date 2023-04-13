cmake_minimum_required(VERSION 3.15)

project(embedded-training)

enable_language(CXX C ASM)

set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

set(CMAKE_TRY_COMPILE_TARGET_TYPE "STATIC_LIBRARY")
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_subdirectory(Drivers)

set(EXECUTABLE Exercise_SPI.elf)

set(GENERATED_FILES 
    Core/Inc/main.h
    Core/Inc/stm32f4xx_hal_conf.h
    Core/Inc/stm32f4xx_it.h


    Core/Src/main.cpp
    Core/StmFiles/syscalls.c
    Core/StmFiles/sysmem.c
    Core/StmFiles/stm32f4xx_hal_msp.c
    Core/StmFiles/stm32f4xx_it.c
    Core/StmFiles/system_stm32f4xx.c
    Core/Startup/startup_stm32f446zetx.s
)

add_executable(${EXECUTABLE} ${GENERATED_FILES})

target_link_libraries(${EXECUTABLE} Drivers)

target_compile_definitions(${EXECUTABLE} PRIVATE
    -DUSE_HAL_DRIVER
    -DSTM32F446xx
)

target_include_directories(${EXECUTABLE} PUBLIC
    Core/Inc
    ${PROJECT_SOURCE_DIR}/Drivers/CMSIS/Device/ST/STM32F4xx/Include
    ${PROJECT_SOURCE_DIR}/Drivers/CMSIS/Include
    ${PROJECT_SOURCE_DIR}/Drivers/STM32F4xx_HAL_Driver/Inc
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
        -T${CMAKE_CURRENT_LIST_DIR}/STM32F401RETX_FLASH.ld
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

# Static code analyzer
add_custom_command(TARGET ${EXECUTABLE}
                    PRE_BUILD
                    COMMAND cppcheck 
                    ${CMAKE_CURRENT_LIST_DIR}/Core/Src/main.cpp 
                    ${CMAKE_CURRENT_LIST_DIR}/Core/Inc/main.h)

# Code formatage                    
add_custom_command(TARGET ${EXECUTABLE}
                    PRE_BUILD
                    COMMAND clang-format
                    -i 
                    ${CMAKE_CURRENT_LIST_DIR}/Core/Src/main.cpp 
                    ${CMAKE_CURRENT_LIST_DIR}/Core/Inc/main.h)
