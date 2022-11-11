cmake_minimum_required(VERSION 3.0.0)
project(simplehttp VERSION 0.1.0)

include_directories("include")

# string(LENGTH "${CMAKE_SOURCE_DIR}/" SOURCE_PATH_SIZE)
# add_definitions("-DSOURCE_PATH_SIZE=${SOURCE_PATH_SIZE}" "-D__FILENAME__=__FILE__ + SOURCE_PATH_SIZE")
add_executable(test src/test.c src/tool.c src/simplehttp.c)
