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
    Execute Command          sysbus LoadELF @${CURDIR}/../../Exercises/4_Exercise_LinkerScript/build/4_Exercise_LinkerScript/Exercise_LinkerScript.elf

*** Test Cases ***
Signature Should Be Changed
    Create Machine           
    Start Emulation
    Sleep                    100ms
    ${signature}              Execute Command    sysbus ReadDoubleWord 0x0801E900
    Should Not Contain           ${signature}    0x444F4F47
