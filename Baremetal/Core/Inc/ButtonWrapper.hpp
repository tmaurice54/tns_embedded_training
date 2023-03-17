#include "stm32f4xx_hal.h"

class ButtonWrapper
{
private:
    GPIO_TypeDef* port;
    uint16_t pin;
public:
    ButtonWrapper(GPIO_TypeDef* port, uint16_t pin);
    ~ButtonWrapper();
    bool isPressed();

};

ButtonWrapper::ButtonWrapper(GPIO_TypeDef* port, uint16_t pin)
{
    this->port = port;
    this->pin = pin;
}

ButtonWrapper::~ButtonWrapper()
{
}

bool ButtonWrapper::isPressed()
{
    return (HAL_GPIO_ReadPin(port, pin) == GPIO_PIN_SET);
}