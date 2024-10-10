cmake_minimum_required(VERSION 3.10)
project(ArtificiusSystem)

set(CMAKE_CXX_STANDARD 17)

find_package(pybind11 REQUIRED)

add_executable(artificius_system main.cpp system.cpp)

target_link_libraries(artificius_system PRIVATE pybind11::embed)
