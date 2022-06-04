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
    glog::glog
    matplot
)
target_include_directories(optical_flow PUBLIC
    ${CERES_INCLUDE_DIRS}
    ${OpenCV_INCLUDE_DIRS}
)
