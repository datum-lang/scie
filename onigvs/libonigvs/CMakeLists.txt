cmake_minimum_required(VERSION 3.1)
project(LibOnigVS LANGUAGES CXX)

#include_directories(oniguruma)
add_subdirectory(oniguruma)

option(BUILD_SHARED_LIBS "Build using shared libraries" ON)

add_library(onigvs STATIC onigvs.cc)

target_link_libraries(onigvs onig)

