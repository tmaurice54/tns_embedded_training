*** Settings ***
Suite Setup                   Setup
Suite Teardown                Teardown
Test Setup                    Reset Emulation
Test Teardown                 Test Teardown
Resource                      ${RENODEKEYWORDS}

*** Keywords ***
Create Machine
    Execute Command          mach create
    Execute Command          machine LoadPlatformDescription @${CURDIR}/../../RenodeConfig/board.repl
    Execute Command          sysbus LoadELF @${CURDIR}/../../Exercises/1_Exercise_GPIO/build/1_Exercise_GPIO/Exercise_GPIO.elf

*** Test Cases ***
Button Led Should Blink
    Create Machine           
    Start Emulation
    Sleep                    100ms
    ${ledstate1}             Execute Command    sysbus.gpioPortA.LD2 State
    Sleep                    1000ms
    ${ledstate2}             Execute Command    sysbus.gpioPortA.LD2 State
    Should Not Be Equal      ${ledstate1}  ${ledstate2}
    Sleep                    1000ms
    ${ledstate3}             Execute Command    sysbus.gpioPortA.LD2 State
    Should Be Equal          ${ledstate1}  ${ledstate3}
