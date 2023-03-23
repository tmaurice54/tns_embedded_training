#include "stm32f4xx_hal.h"

class GpioWrapper
{
private:
    GPIO_TypeDef* GPIOport;
    uint32_t GPIOpin;

public:
    GpioWrapper(GPIO_TypeDef* GPIOport, uint32_t GPIOpin);
    GpioSet();
    GpioReset();
    GpioRead();

    ~GpioWrapper();
};

GpioWrapper::GpioWrapper(GPIO_TypeDef* GPIOport, uint32_t GPIOpin)
{
    this->GPIOport = GPIOport;
    this->GPIOpin = GPIOpin;
}

GpioWrapper::GpioSet()
{

}

GpioWrapper::GpioReset()
{

}

GpioWrapper::GpioRead()
{

}

GpioWrapper::~GpioWrapper()
{
}
