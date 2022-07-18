#ifndef MIRAGE_UTIL_H
#define MIRAGE_UTIL_H

#include <vector>

#include <opencv2/opencv.hpp>

namespace util {

template <typename T>
void prune_vector(std::vector<T>& vec, std::vector<unsigned char>& mask) {
    assert(vec.size() == mask.size());

    std::vector<T> old = vec;

    vec.clear();
    vec.reserve(mask.size());

    for (size_t i=0; i < mask.size(); i++) {
        if (!mask[i]) { continue; }

        vec.push_back(old[i]);
    }
}

template <typename T>
void prune_vector(std::vector<T>& vec, cv::Mat& mask) {
    assert(vec.size() == mask.rows);

    std::vector<T> old = vec;

    vec.clear();
    vec.reserve(mask.rows);

    for (size_t i=0; i < mask.rows; i++) {
        if (mask.at<unsigned char>(i, 0) == 0) { continue; }

        vec.push_back(old[i]);
    }
}

}

#endif  // MIRAGE_UTIL_H