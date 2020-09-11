cmake_minimum_required(VERSION 3.1)
project(onigvs VERSION 6.9.5 LANGUAGES CXX)

add_library(onigvs STATIC onig.cc)

install(TARGETS onigvs DESTINATION .)
