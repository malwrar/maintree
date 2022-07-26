#ifndef MIRAGE_MATH_H
#define MIRAGE_MATH_H

#include <vector>

#include <opencv2/opencv.hpp>

namespace math {

bool triangulateF(
    std::vector<cv::Point2f> p1,
    std::vector<cv::Point2f> p2,
    cv::Mat F,
    cv::Mat K,
    cv::Mat& R21,
    cv::Mat& T21,
    std::vector<cv::Point3f>& p3d
);

}

#endif  // MIRAGE_MATH_H