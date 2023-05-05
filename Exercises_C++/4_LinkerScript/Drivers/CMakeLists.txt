set(MODULE_NAME Drivers)

set(STM_HAL_SOURCES 
    STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_tim.c
    STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_tim_ex.c
    STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_uart.c
    STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_rcc.c
    STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_rcc_ex.c
    STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_flash.c
    STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_flash_ex.c
    STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_flash_ramfunc.c
    STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_gpio.c
    STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_dma_ex.c
    STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_dma.c
    STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_pwr.c
    STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_pwr_ex.c
    STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_cortex.c
    STM32F4xx_HAL_Driver/Src/stm32f4xx_hal.c
    STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_exti.c
    STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_pcd.c
    STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_pcd_ex.c
    STM32F4xx_HAL_Driver/Src/stm32f4xx_ll_usb.c
    STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_usart.c
    STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_adc.c
    STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_adc_ex.c
    STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_eth.c
    STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_i2c.c
    STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_i2c_ex.c
    STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_spi.c    
)

add_library(${MODULE_NAME} ${STM_HAL_SOURCES})

target_include_directories(${MODULE_NAME} PUBLIC
    CMSIS/Device/ST/STM32F4xx/Include
    CMSIS/Include
    STM32F4xx_HAL_Driver/Inc
)

target_compile_definitions(${MODULE_NAME} PRIVATE
    -DUSE_HAL_DRIVER
    -DSTM32F446xx
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