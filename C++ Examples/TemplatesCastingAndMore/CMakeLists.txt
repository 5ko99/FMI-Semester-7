# CMakeList.txt : CMake project for TemplatesCastingAndMore, include source and define
# project specific logic here.
#
cmake_minimum_required (VERSION 3.8)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

project ("TemplatesCastingAndMore")

# Add source to this project's executable.
add_executable (TemplatesCastingAndMore "main.cpp" )

# TODO: Add tests and install targets if needed.
