cmake_minimum_required(VERSION 3.10)
project(Perfcon)

set(CMAKE_CXX_STANDARD 17)

find_package(pybind11 REQUIRED)

add_executable(Perfcon main.cpp system.cpp)

target_link_libraries(Perfcon PRIVATE pybind11::embed)
