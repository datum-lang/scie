cmake_minimum_required(VERSION 3.1)
project(LibOnigVS LANGUAGES CXX)

add_library(onigvs STATIC onig.cc)

include_directories(../../rust-onig/onig_sys/oniguruma/src)

file(GLOB LIBRARIES "../../rust-onig/onig_sys/oniguruma/libonig.dylib")
message("LIBRARIES = ${LIBRARIES}")

install(TARGETS onigvs DESTINATION .)
