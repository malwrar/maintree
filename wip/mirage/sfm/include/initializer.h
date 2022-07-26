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
    Initializer(cv::Mat initial_keyframe, cv::Mat K) : K(K) {
        setKeyframe(initial_keyframe);
    }

    bool attemptTriangulation(cv::Mat frame, cv::Mat& Rcw, cv::Mat& Tcw, std::vector<cv::Point3f>& p3d);
    void setKeyframe(cv::Mat keyframe);

    cv::Mat createDebugImage();

private:
    cv::Mat K;
    std::vector<cv::Mat> keyframe_pyr, last_pyr;
    std::vector<cv::Point2f> keyframe_points, last_points;

    std::vector<cv::Mat> preprocessFrame(const cv::Mat& frame);
    bool triangulate(std::vector<cv::Point2f> p1, std::vector<cv::Point2f> p2,
        cv::Mat& Rcw, cv::Mat& Tcw, std::vector<cv::Point3f>& p3);
};

#endif  // MIRAGE_INITIALIZER_H