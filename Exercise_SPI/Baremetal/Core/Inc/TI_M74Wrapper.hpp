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
    float readTemp();
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

float TI_M74Wrapper::readTemp()
{
    uint8_t data[2];
    uint16_t temp, tempIf;
    HAL_GPIO_WritePin (GPIOB, GPIO_PIN_6, GPIO_PIN_RESET);  // pull the pin low
	HAL_SPI_Receive (&hspi, data, 2, 100);  // receive 2 bytes data
	HAL_GPIO_WritePin (GPIOB, GPIO_PIN_6, GPIO_PIN_SET);  // pull the pin high
    temp = ((uint16_t)data[0] << 8) | data[1];
    tempIf = temp;
    if((tempIf >> 15)==1){
        return -(((~temp >> 3)+0b0000000000000001)*0.0625);
    } else {
        return (temp >> 3)*0.0625;
    }
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
