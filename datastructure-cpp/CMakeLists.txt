cmake_minimum_required (VERSION 3.8)

project ("neu-datastructure-cpp")

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

include_directories("include")
include_directories("dep/fmt/include")

aux_source_directory("dep/fmt/src" FMT_SRCS)
file(GLOB LABS_SRCS "src/lab*/*.cpp")

add_executable (neu-datastructure-cpp "src/main.cpp" ${FMT_SRCS} ${LABS_SRCS})

