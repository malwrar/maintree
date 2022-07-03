cmake_minimum_required(VERSION 3.18)
project(atlas)

find_package(Ceres REQUIRED)
find_package(glog REQUIRED)
find_package(OpenCV REQUIRED)

add_library(atlas STATIC)
target_compile_features(atlas PRIVATE cxx_std_17)

target_sources(atlas
    PRIVATE
        src/atlas.cpp
)

target_include_directories(atlas
    PRIVATE
        # Where the library itself will look for its internal headers
        ${CMAKE_CURRENT_SOURCE_DIR}/src
    PUBLIC
        # Where top-level project will look for the library's public headers
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
        # Where external projects will look for the library's public headers
        $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>
)


target_link_libraries(atlas PUBLIC
    ${CERES_LIBRARIES}
    ${OpenCV_LIBS}
    glog::glog
)

target_include_directories(atlas PUBLIC
    ${CERES_INCLUDE_DIRS}
    ${OpenCV_INCLUDE_DIRS}
)

add_executable(visualizer src/bin/visualizer.cpp)
target_link_libraries(visualizer PUBLIC atlas)
target_include_directories(visualizer PUBLIC ${OpenCV_INCLUDE_DIRS})
target_link_libraries(visualizer PUBLIC ${OpenCV_LIBS})