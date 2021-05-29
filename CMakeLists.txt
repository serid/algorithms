cmake_minimum_required(VERSION 3.19)

project(algorithms_cxx CXX)
set(CMAKE_CXX_STANDARD 20)
add_executable(algorithms_cxx cxx/main.cpp cxx/utils.cpp cxx/utils.hpp cxx/radix_sort.cpp cxx/radix_sort.hpp cxx/ack.cpp cxx/ack.hpp cxx/aes/aes.cpp cxx/aes/aes.hpp cxx/aes/mat.hpp)

project(algorithms_c C)
set(CMAKE_C_STANDARD 11)
add_executable(algorithms_c c/gc/gc.c c/gc/gc.h c/vec.h c/ints.h c/main.c c/gc/expr.c c/gc/expr.h c/util.c c/util.h)
