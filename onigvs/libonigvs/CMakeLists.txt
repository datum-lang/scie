cmake_minimum_required(VERSION 3.1)
project(LibOnigVS LANGUAGES CXX)

add_library(onigvs STATIC onig.cc)

install(TARGETS onigvs DESTINATION .)
