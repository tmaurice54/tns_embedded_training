cmake_minimum_required(VERSION 3.15)

project(embedded-training)

enable_language(CXX C ASM)

set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

set(CMAKE_TRY_COMPILE_TARGET_TYPE "STATIC_LIBRARY")
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

if(${EXERCICE} STREQUAL "HELLOWORLD")
    add_subdirectory(0_HelloWorld)
elseif(${EXERCICE} STREQUAL "GPIO")
    add_subdirectory(1_GPIO)
elseif(${EXERCICE} STREQUAL "Sensor")
    add_subdirectory(2_Sensor)
elseif(${EXERCICE} STREQUAL "FreeRTOS")
    add_subdirectory(3_FreeRTOS)
    add_subdirectory(Global_libraries/Middlewares)
elseif(${EXERCICE} STREQUAL "LINKERSCRIPT")
    add_subdirectory(4_LinkerScript)
elseif(${EXERCICE} STREQUAL "DEBUG")
    add_subdirectory(5_Debug)
elseif(${EXERCICE} STREQUAL "ADC")
    add_subdirectory(6_ADC)
else()
    message(FATAL_ERROR "Please select an exercise")
endif()

add_subdirectory(Global_libraries/Drivers)