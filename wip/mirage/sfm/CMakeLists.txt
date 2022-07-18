cmake_minimum_required(VERSION 3.18)
project(mirage)

find_package(Ceres REQUIRED)
find_package(glog REQUIRED)
find_package(OpenCV REQUIRED)

# Main library
add_library(mirage STATIC)

target_sources(mirage
    PRIVATE
        src/initializer.cpp
        src/util.cpp
)

target_include_directories(mirage
    PRIVATE
        # where the library itself will look for its internal headers
        ${CMAKE_CURRENT_SOURCE_DIR}/src
    PUBLIC
        # where top-level project will look for the library's public headers
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
        # where external projects will look for the library's public headers
        $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>
)

target_compile_features(mirage PRIVATE cxx_std_17)

target_link_libraries(mirage PUBLIC
    ${CERES_LIBRARIES}
    ${OpenCV_LIBS}
    glog::glog
)

target_include_directories(mirage PUBLIC
    ${CERES_INCLUDE_DIRS}
    ${OpenCV_INCLUDE_DIRS}
)

# Visualizer demo bin
add_executable(visualizer src/bin/visualizer.cpp)
target_link_libraries(visualizer PUBLIC mirage ${OpenCV_LIBS})
target_include_directories(visualizer PUBLIC ${OpenCV_INCLUDE_DIRS})