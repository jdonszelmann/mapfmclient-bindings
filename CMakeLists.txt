cmake_minimum_required(VERSION 3.19)
project(mapfmclient_bindings CXX)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)
#set(CMAKE_VERBOSE_MAKEFILE ON)

include_directories(.)
include_directories(test)

link_directories(${CMAKE_CURRENT_SOURCE_DIR}/target/debug/)


add_executable(test
        bindings.h
        test/main.cpp
)
#target_link_directories(mapfmclient_bindings PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/target/debug/)
target_link_libraries(test mapfmclient_bindings)
