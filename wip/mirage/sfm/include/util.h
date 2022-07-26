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

size_t count_nonzero(std::vector<unsigned char>& mask);

size_t count_nonzero(cv::Mat& mask);

void print_matrix(cv::Mat mat, const char* label);

void form_intrinsic_matrices(
    cv::Mat& K,
    cv::Mat& dist_coef,
    double fx,
    double fy,
    double cx,
    double cy,
    double k1,
    double k2,
    double p1,
    double p2
);

bool decomposeE(
    cv::Mat E,
    cv::Mat R1,
    cv::Mat R2,
    cv::Mat t1,
    cv::Mat t2
);

bool triangulate(
    const std::vector<cv::Point2f>& p1,
    const std::vector<cv::Point2f>& p2,
    std::vector<cv::Point3d>& p3,
    cv::Mat E
);

bool triangulate(
    const std::vector<cv::Point2f>& p1,
    const std::vector<cv::Point2f>& p2,
    std::vector<cv::Point3d>& p3,
    cv::Mat_<double> R1,
    cv::Mat_<double> R2,
    cv::Mat_<double> t1,
    cv::Mat_<double> t2
);

bool triangulate(
    const std::vector<cv::Point2f>& p1,
    const std::vector<cv::Point2f>& p2,
    std::vector<cv::Point3d>& p3,
    cv::Mat_<double> R,
    cv::Mat_<double> t
);

}

#endif  // MIRAGE_UTIL_H