#ifndef MIRAGE_INITIALIZER_H
#define MIRAGE_INITIALIZER_H

#include <vector>
#include <utility>

#include <opencv2/opencv.hpp>

/**
 * @brief Self-contained map initializer for monocular sequences.
 * 
 * This class manages the initial triangulation of 2d features tracked between
 * a keyframe and some subsequent frame with sufficient parallax to ensure
 * accurate location.
 */
class Initializer {
public:
    Initializer(
        cv::Mat initial_keyframe,
        cv::Mat K,
        cv::Mat dist_coef,
        int pyramid_depth=4
    ) : K(K), dist_coef(dist_coef), pyramid_depth(pyramid_depth) {
        setKeyframe(initial_keyframe);
        K.convertTo(K, CV_32F);
    }

    bool attemptTriangulation(cv::Mat frame,
        cv::Mat& Rcw, cv::Mat& Tcw, std::vector<cv::Point3f>& p3d);
    void setKeyframe(cv::Mat keyframe);

    cv::Mat createDebugImage();

private:
    cv::Mat K, dist_coef;
    std::vector<cv::Mat> keyframe_pyr, last_pyr;
    std::vector<cv::Point2f> keyframe_points, last_points;
    int pyramid_depth;

    std::vector<cv::Mat> preprocessFrame(const cv::Mat& frame);
    bool attemptTriangulation(
        std::vector<cv::Mat> pyr1, std::vector<cv::Point2f> pt1,
        std::vector<cv::Mat> pyr2, std::vector<cv::Point2f> pt2,
        cv::Mat& Rcw, cv::Mat& Tcw, std::vector<cv::Point3f>& p3d);
};

#endif  // MIRAGE_INITIALIZER_H