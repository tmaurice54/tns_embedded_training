set(EXECUTABLE LEDFreeRTOS.out)

set(GENERATED_FILES

    Core/Inc/main.h
    Core/Inc/stm32f4xx_hal_conf.h
    Core/Inc/stm32f4xx_it.h

    Core/Src/main.cpp
    Core/Src/freertos.c
    Core/Src/syscalls.c
    Core/Src/sysmem.c
    Core/Src/stm32f4xx_hal_msp.c
    Core/Src/stm32f4xx_hal_timebase_tim.c
    Core/Src/stm32f4xx_it.c
    Core/Src/system_stm32f4xx.c
    Core/Startup/startup_stm32f446zetx.s
)


add_executable(${EXECUTABLE} ${GENERATED_FILES})

target_link_libraries(${EXECUTABLE} Drivers Middlewares)


target_compile_definitions(${EXECUTABLE} PRIVATE
    -DUSE_HAL_DRIVER
    -DSTM32F446xx
    -DDEBUG
)

target_include_directories(${EXECUTABLE} PUBLIC
    Core/Inc
    ${PROJECT_SOURCE_DIR}/Drivers/CMSIS/Device/ST/STM32F4xx/Include
    ${PROJECT_SOURCE_DIR}/Drivers/CMSIS/Include
    ${PROJECT_SOURCE_DIR}/Drivers/STM32F4xx_HAL_Driver/Inc
    ${PROJECT_SOURCE_DIR}/Middlewares/Third_Party/FreeRTOS/Source/include
    ${PROJECT_SOURCE_DIR}/Middlewares/Third_Party/FreeRTOS/Source/CMSIS_RTOS_V2
    ${PROJECT_SOURCE_DIR}/Middlewares/Third_Party/FreeRTOS/Source/portable/GCC/ARM_CM4F
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
    -T${CMAKE_CURRENT_LIST_DIR}/STM32F446ZETX_FLASH.ld
    -mcpu=cortex-m4
    -mthumb
    -mfpu=fpv4-sp-d16
    -mfloat-abi=hard
    -specs=nano.specs
    -lc
    -lm
    -Wl,-Map=${EXECUTABLE}.map,--cref
    -Wl,--gc-sections
)



# Print executable size
add_custom_command(TARGET ${EXECUTABLE}
        POST_BUILD
        COMMAND arm-none-eabi-size ${EXECUTABLE})

# Create hex file
add_custom_command(TARGET ${EXECUTABLE}
        POST_BUILD
        COMMAND arm-none-eabi-objcopy -O ihex ${EXECUTABLE} ${PROJECT_NAME}.hex
        COMMAND arm-none-eabi-objcopy -O binary ${EXECUTABLE} ${PROJECT_NAME}.bin)
