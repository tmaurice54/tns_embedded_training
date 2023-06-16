# Debug using UART

The UART protocol can be used to print data to the PC trough the STLink.
We just must use the UART2 on the board STM32F401RE.

## Print on serial terminal with UART (C++ version)

In the main files given in this training you will find initialization for the UART2. This allows you to use it as a debug tool (or just to print data if you want to).  

You can use the following code:  

```cpp
#include <stdio.h>
/*
CODE
*/
#ifdef __GNUC__
#define PUTCHAR_PROTOTYPE int __io_putchar(int ch)
#else
#define PUTCHAR_PROTOTYPE int fputc(int ch, FILE *f)
#endif

PUTCHAR_PROTOTYPE {
  HAL_UART_Transmit(&huart2, (uint8_t *)&ch, 1, HAL_MAX_DELAY);
  return ch;
}
/*
CODE
*/
printf("%s\r\n", bufferToWrite);
```

This code will rewrite the `fputc()` function wich is used by the `printf()` function.
Now your prints are redirect to the UART2.
You can then use a serial terminal such as putty and read the data printed.

You can also directly use the function `HAL_UART_Transmit` to print the text you want.

## Print on serial terminal with UART (Rust version)

To print data using UART in rust you just have to add the UART set up and then you can write data using the macro `writeln!`.

```rust
    // Macro to write data
    use core::fmt::Write;

    // HAL library for stm32f4xx board
    use stm32f4xx_hal::{
        pac,
        prelude::*,
        serial::{config::Config, Serial},
    };

    // CODE
    
    // Set TX pin
    let tx_pin = gpioa.pa2.into_alternate();

    // Set UART2
    let mut tx = Serial::tx(
        device.USART2,
        tx_pin,
        Config::default()
                  .baudrate(115200.bps())
                  .wordlength_8()
                  .parity_none(),
        &clocks,
        )
      .unwrap();

    writeln!(tx, "Hello World !\r").unwrap();
```
