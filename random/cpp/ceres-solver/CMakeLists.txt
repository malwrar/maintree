cmake_minimum_required(VERSION 3.18)

find_package(Ceres REQUIRED)
find_package(glog REQUIRED)

# http://ceres-solver.org/nnls_tutorial.html#hello-world
add_executable(helloworld src/helloworld.cpp)
target_compile_features(helloworld PRIVATE cxx_std_17)
target_link_libraries(helloworld PUBLIC
    glog::glog
    ${CERES_LIBRARIES}
)
target_include_directories(helloworld PUBLIC
    ${CERES_INCLUDE_DIRS}
)
