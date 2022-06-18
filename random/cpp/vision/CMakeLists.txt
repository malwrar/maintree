cmake_minimum_required(VERSION 3.18)
project(ceres-examples)

add_subdirectory(../../../third_party/cpp/matplotplusplus ./matplot)

find_package(Ceres REQUIRED)
find_package(glog REQUIRED)
find_package(OpenCV REQUIRED)

add_executable(featurestuff src/featurestuff.cpp)
target_compile_features(featurestuff PRIVATE cxx_std_17)
target_link_libraries(featurestuff PUBLIC
    ${CERES_LIBRARIES}
    ${OpenCV_LIBS}
    glog::glog
    matplot
)
target_include_directories(featurestuff PUBLIC
    ${CERES_INCLUDE_DIRS}
    ${OpenCV_INCLUDE_DIRS}
)

add_executable(optical_flow src/optical_flow.cpp)
target_compile_features(optical_flow PRIVATE cxx_std_17)
target_link_libraries(optical_flow PUBLIC
    ${CERES_LIBRARIES}
    ${OpenCV_LIBS}
)
target_include_directories(optical_flow PUBLIC
    ${CERES_INCLUDE_DIRS}
    ${OpenCV_INCLUDE_DIRS}
)

add_library(vision STATIC)
target_sources(vision
    PRIVATE
        src/vision.cpp
)
target_include_directories(vision
    PRIVATE
        # where the library itself will look for its internal headers
        ${CMAKE_CURRENT_SOURCE_DIR}/src
    PUBLIC
        # where top-level project will look for the library's public headers
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
        # where external projects will look for the library's public headers
        $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>
)
target_compile_features(vision PRIVATE cxx_std_17)
target_link_libraries(vision PUBLIC
    ${CERES_LIBRARIES}
    ${OpenCV_LIBS}
    glog::glog
    matplot
)
target_include_directories(vision PUBLIC
    ${CERES_INCLUDE_DIRS}
    ${OpenCV_INCLUDE_DIRS}
)

add_executable(incremental_sfm src/incremental_sfm.cpp)
target_link_libraries(incremental_sfm PUBLIC vision)
target_include_directories(incremental_sfm PUBLIC ${OpenCV_INCLUDE_DIRS})
target_link_libraries(incremental_sfm PUBLIC ${OpenCV_LIBS})

add_executable(8point src/8_point.cpp)
target_link_libraries(8point PUBLIC vision)
target_include_directories(8point PUBLIC ${OpenCV_INCLUDE_DIRS})
target_link_libraries(8point PUBLIC ${OpenCV_LIBS})
