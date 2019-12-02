cmake_minimum_required(VERSION 3.14)
project(cubeage)

set(CMAKE_CXX_STANDARD 14)

include_directories(.)
include_directories(target/debug/build/bzip2-sys-9a83aad92a7adaef/out/include)

add_executable(cubeage Cargo.toml)