set(MODULE_NAME Middlewares)

set(MIDDLEWARES_FILES
    Third_Party/FreeRTOS/Source/portable/MemMang/heap_4.c
    Third_Party/FreeRTOS/Source/portable/GCC/ARM_CM4F/port.c
    Third_Party/FreeRTOS/Source/CMSIS_RTOS_V2/cmsis_os2.c
    Third_Party/FreeRTOS/Source/croutine.c
    Third_Party/FreeRTOS/Source/event_groups.c
    Third_Party/FreeRTOS/Source/list.c
    Third_Party/FreeRTOS/Source/queue.c
    Third_Party/FreeRTOS/Source/stream_buffer.c
    Third_Party/FreeRTOS/Source/tasks.c
    Third_Party/FreeRTOS/Source/timers.c
)

add_library(${MODULE_NAME} ${MIDDLEWARES_FILES})

target_include_directories(${MODULE_NAME} PUBLIC
    ${PROJECT_SOURCE_DIR}/FreeRTOS/Core/Inc
    ${PROJECT_SOURCE_DIR}/Drivers/CMSIS/Device/ST/STM32F4xx/Include
    ${PROJECT_SOURCE_DIR}/Drivers/CMSIS/Include
    ${PROJECT_SOURCE_DIR}/Drivers/STM32F4xx_HAL_Driver/Inc
    Third_Party/FreeRTOS/Source/include
    Third_Party/FreeRTOS/Source/CMSIS_RTOS_V2
    Third_Party/FreeRTOS/Source/portable/GCC/ARM_CM4F
)

target_compile_options(${MODULE_NAME} PRIVATE
    -mcpu=cortex-m4    
    -ffunction-sections
    -fdata-sections 
    -Wall 
    -fstack-usage 
    -mfpu=fpv4-sp-d16 
    -mfloat-abi=hard 
    -mthumb
)
target_compile_definitions(${MODULE_NAME} PRIVATE
    -DUSE_HAL_DRIVER
    -DSTM32F446xx
)
target_link_options(${MODULE_NAME} PRIVATE
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