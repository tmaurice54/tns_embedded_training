*** Settings ***
Suite Setup                   Setup
Suite Teardown                Teardown
Test Setup                    Reset Emulation
Test Teardown                 Test Teardown
Resource                      ${RENODEKEYWORDS}


*** Keywords ***
Create Machine
    [Arguments]              ${hex_file}
    Execute Command          mach create
    Execute Command          machine LoadPlatformDescription @${CURDIR}\\..\\renode\\board.repl
    Execute Command          sysbus LoadELF @${hex_file}
    Execute Command          sysbus Tag <0x40023C00, 0x40023C07> "FLASH:ACR" 0x5
    Execute Command          sysbus Tag <0x50000010, 0x5000003f> "USB:RESET" 0x80000000

*** Test Cases ***
Button Press Should Turn Led On
    Create Machine           @${CURDIR}/../../../build/Base/FreeRTOS/FreeRTOS.out
    Start Emulation
    Sleep                    100ms
    Execute Command          sysbus.gpioPortC.UserButton Press
    Sleep                    100ms
    ${ledstate}=   Execute Command    sysbus.gpioPortB.LD1 State
    Should Contain           ${ledstate}    True

Button PressAndRelease Should Turn Led Off
    Create Machine           @${CURDIR}/../../../build/Base/FreeRTOS/FreeRTOS.out
    Start Emulation
    Sleep                    100ms
    Execute Command          sysbus.gpioPortC.UserButton Press
    Sleep                    100ms
    Execute Command          sysbus.gpioPortC.UserButton Release
    Sleep                    100ms
    ${ledstate}=   Execute Command    sysbus.gpioPortB.LD1 State
    Should Contain           ${ledstate}    False
