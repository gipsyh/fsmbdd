cmake_minimum_required(VERSION 3.22)

project(fsmbdd_lace)

add_library(fsmbdd_lace src/fsmbdd.c)

set(SYLVAN "${CMAKE_CURRENT_SOURCE_DIR}/../../sylvan")
add_subdirectory(${SYLVAN} sylvan)
target_include_directories(fsmbdd_lace PUBLIC ${SYLVAN}/src)
target_link_directories(fsmbdd_lace PUBLIC ${SYLVAN}/build/src)
target_link_libraries(fsmbdd_lace PUBLIC sylvan)

install(TARGETS fsmbdd_lace
        LIBRARY DESTINATION lib
        PUBLIC_HEADER DESTINATION include
        )