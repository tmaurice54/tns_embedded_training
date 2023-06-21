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
    Execute Command          sysbus LoadELF @${CURDIR}/../../Exercises_Rust/3_LinkerScript/target/thumbv7em-none-eabihf/release/linker_script

*** Test Cases ***
Signature Should Be Changed
    Create Machine           
    Start Emulation
    Sleep                    100ms
    ${signature}              Execute Command    sysbus ReadDoubleWord 0x0801E900
    Should Not Contain           ${signature}    0x444F4F47
