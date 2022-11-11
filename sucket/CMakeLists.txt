cmake_minimum_required(VERSION 3.0.0)
project(sucket VERSION 0.1.0)

#include(CTest)
#enable_testing()

set(NETUTIL_SRC src/netutil.c)
#include_directories("include")
#include_directories("${PROJECT_SOURCE_DIR}/include")
#aux_source_directory("src" SRCS)
#aux_source_directory("src/timeserver" TIME_SRCS)
add_executable(teeest src/main.c ${NETUTIL_SRC})

add_executable(timeclient src/timeserver/client.c ${NETUTIL_SRC})
add_executable(timeserver src/timeserver/server.c ${NETUTIL_SRC})

add_executable(fileclient src/fileserver/client.c src/fileserver/util.c ${NETUTIL_SRC})
add_executable(fileserver src/fileserver/server.c src/fileserver/util.c ${NETUTIL_SRC})

target_include_directories(teeest PUBLIC "include")

target_include_directories(timeclient PUBLIC "include")
target_include_directories(timeserver PUBLIC "include")

target_include_directories(fileclient PUBLIC "include")
target_include_directories(fileserver PUBLIC "include")

#set(CPACK_PROJECT_NAME ${PROJECT_NAME})
#set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
#include(CPack)
