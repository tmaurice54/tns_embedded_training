#ifndef __TI_M74Wrapper_HPP
#define __TI_M74Wrapper_HPP

#include "stm32f4xx_hal.h"

class TI_M74Wrapper
{
private:
    SPI_HandleTypeDef hspi;
public:
    TI_M74Wrapper(SPI_HandleTypeDef hspi);
    void init();
    uint16_t readTemp();
    void writeData(uint8_t address, uint8_t value);
    ~TI_M74Wrapper();
};

TI_M74Wrapper::TI_M74Wrapper(SPI_HandleTypeDef hspi)
{
	this->hspi = hspi;
}

void TI_M74Wrapper::init()
{
    this->writeData(0x00, 0x00);
}

uint16_t TI_M74Wrapper::readTemp()
{
    uint8_t data[2];
    HAL_GPIO_WritePin (GPIOB, GPIO_PIN_6, GPIO_PIN_RESET);  // pull the pin low
	HAL_SPI_Receive (&hspi, data, 2, 100);  // receive 6 bytes data
	HAL_GPIO_WritePin (GPIOB, GPIO_PIN_6, GPIO_PIN_SET);  // pull the pin high
    uint16_t ret = ((uint16_t)data[0] << 8) | data[1];
    return ret;
}

void TI_M74Wrapper::writeData(uint8_t address, uint8_t value)
{
	uint8_t data[2];
	data[0] = address|0x40;  // multibyte write
	data[1] = value;
	HAL_GPIO_WritePin (GPIOB, GPIO_PIN_6, GPIO_PIN_RESET);  // pull the cs pin low
	HAL_SPI_Transmit (&hspi, data, 2, 100);  // write data to register
	HAL_GPIO_WritePin (GPIOB, GPIO_PIN_6, GPIO_PIN_SET);  // pull the cs pin high
}

TI_M74Wrapper::~TI_M74Wrapper()
{
}

#endif
