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
    Execute Command          machine LoadPlatformDescription @${CURDIR}/../../RenodeConfig/board.repl
    Execute Command          sysbus LoadELF @${hex_file}
    Execute Command          sysbus Tag <0x40023C00, 0x40023C07> "FLASH:ACR" 0x5
    Execute Command          sysbus Tag <0x50000010, 0x5000003f> "USB:RESET" 0x80000000

*** Test Cases ***
Button Led Should Blink
    Create Machine           @${CURDIR}/../../Exercises/1_Exercise_GPIO/build/1_Exercise_GPIO/Exercise_GPIO.elf
    Start Emulation
    Sleep                    100ms
    ${ledstate1}             Execute Command    sysbus.gpioPortA.LD2 State
    Sleep                    1000ms
    ${ledstate2}             Execute Command    sysbus.gpioPortA.LD2 State
    Should Not Be Equal      ${ledstate1}  ${ledstate2}
    Sleep                    1000ms
    ${ledstate3}             Execute Command    sysbus.gpioPortA.LD2 State
    Should Be Equal          ${ledstate1}  ${ledstate3}
