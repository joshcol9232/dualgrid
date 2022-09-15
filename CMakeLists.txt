cmake_minimum_required(VERSION 3.5)

project(dualgrid LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(OpenMP REQUIRED)

add_executable(dualgrid main.cpp
    constructionset.h
    vector.h
    intersection.h)
#    state.h state.cpp)

target_link_libraries(dualgrid PRIVATE OpenMP::OpenMP_CXX)
