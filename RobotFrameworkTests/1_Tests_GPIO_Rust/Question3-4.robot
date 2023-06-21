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
    Execute Command          sysbus LoadELF @${CURDIR}/../../Exercises_Rust/1_GPIO/target/thumbv7em-none-eabihf/release/GPIO

*** Test Cases ***
Button Press Should Turn Led On
    Create Machine           
    Start Emulation
    Sleep                    100ms
    Execute Command          sysbus.gpioPortC.UserButton Press
    Sleep                    100ms
    ${ledstate}=   Execute Command    sysbus.gpioPortA.LD2 State
    Should Contain           ${ledstate}    True

Button PressAndRelease Should Turn Led Off
    Create Machine
    Start Emulation
    Sleep                    100ms
    Execute Command          sysbus.gpioPortC.UserButton Press
    Sleep                    100ms
    Execute Command          sysbus.gpioPortC.UserButton Release
    Sleep                    100ms
    ${ledstate}=   Execute Command    sysbus.gpioPortA.LD2 State
    Should Contain           ${ledstate}    False
