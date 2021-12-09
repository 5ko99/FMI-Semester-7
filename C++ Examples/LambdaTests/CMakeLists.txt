# CMakeList.txt : CMake project for LambdaTests, include source and define
# project specific logic here.
#
cmake_minimum_required (VERSION 3.8)

project ("LambdaTests")

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)


add_executable (main 
"LambdaTests.cpp")