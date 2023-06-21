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
    Execute Command          sysbus LoadELF @${CURDIR}/../../Exercises_Rust/2_Sensor/target/thumbv7em-none-eabihf/release/Sensor

*** Test Cases ***
Button Led Should Be On
    Create Machine           
    Start Emulation
    Sleep                    100ms
    ${ledstate}              Execute Command    sysbus.gpioPortA.LD2 State
    Should Contain           ${ledstate}    False
    Sleep                    100ms
    Execute Command          sysbus.spi2.sensor Temperature 50
    Sleep                    100ms
    ${ledstate}              Execute Command    sysbus.gpioPortA.LD2 State
    Should Contain           ${ledstate}    True 
