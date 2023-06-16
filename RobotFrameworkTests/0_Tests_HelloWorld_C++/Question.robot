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
    Execute Command          sysbus LoadELF @${CURDIR}/../../Exercises_C++/0_HelloWorld/build/0_HelloWorld/Exercise_HelloWorld.elf

*** Test Cases ***
Button Led Should Blink
    Create Machine           
    Start Emulation
    ${ledstate1}             Execute Command    sysbus.gpioPortA.LD2 State
    Sleep                    100ms
    Log                      ${ledstate1}
    Sleep                    1400ms
    ${ledstate2}             Execute Command    sysbus.gpioPortA.LD2 State
    Log                      ${ledstate2}
    Should Not Be Equal      ${ledstate1}  ${ledstate2}
    Sleep                    1500ms
    ${ledstate3}             Execute Command    sysbus.gpioPortA.LD2 State
    Should Be Equal          ${ledstate1}  ${ledstate3}
