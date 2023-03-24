cmake_minimum_required(VERSION 3.15)

project(embedded-training)

enable_language(CXX C ASM)

set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

set(CMAKE_TRY_COMPILE_TARGET_TYPE "STATIC_LIBRARY")

add_subdirectory(Middlewares)
add_subdirectory(Drivers)
add_subdirectory(Base)
add_subdirectory(Exercise_LED)
add_subdirectory(Exercise_SPI)
