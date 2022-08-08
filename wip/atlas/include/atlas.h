#ifndef ATLAS_H
#define ATLAS_H

#include <vector>

#include <opencv2/opencv.hpp>
#include <opencv2/imgproc.hpp>
#include <opencv2/line_descriptor.hpp>

void a();

class Frame {
public:
    Frame(cv::Mat& image);

    Frame retrack(cv::Mat& image);

    cv::Mat describeFeatures();
    void findMatchingFeatureIndices(Frame& frame);

    cv::Mat image;
    std::vector<cv::KeyPoint> features;

private:
    Frame(cv::Mat& image, std::vector<cv::KeyPoint> features);

    cv::Mat preprocessImage(cv::Mat& image);
};

/**
 * @brief Relates keyframes via their shared correspondence.
 * 
 * This class is used to help track key milestone views into a scene
 */
class ViewGraph {
public:
    ViewGraph();

private:
};

#endif  // ATLAS_H