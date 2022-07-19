#include "initializer.h"
#include "util.h"

static cv::Ptr<cv::CLAHE> CLAHE = cv::createCLAHE();
static cv::Ptr<cv::Feature2D> FAST = cv::FastFeatureDetector::create(10);

bool Initializer::attemptTriangulation(
    cv::Mat frame,
    std::vector<cv::Point3d>& points_3d
) {
    // We need at least 5 points to do a triangulation, we failed to (re)find
    // enough we should create a new keyframe.
    if (last_points.size() < 5) {
        setKeyframe(frame);
        return false;
    }

    std::vector<cv::Mat> pyramid = preprocessFrame(frame);

    // Attempt to refind points from the last frame using optical flow.
    std::vector<cv::Point2f> retracked_points(last_points.size());
    std::vector<unsigned char> status(last_points.size());
    std::vector<float> error(last_points.size());

    cv::calcOpticalFlowPyrLK(last_pyr, pyramid, last_points,
            retracked_points, status, error, cv::Size(9, 9), 4);

    util::prune_vector(retracked_points, status);
    util::prune_vector(last_points, status);

    // TODO: backwards track & prune any that are untracked or some # of pixels off

    // TODO: attempt triangulation

    // Get ready for the next invocation.
    last_pyr = pyramid;
    last_points = retracked_points;

    return false;
}

/**
 * @brief (re)starts initialization at the provided keyframe.
 * 
 * @param keyframe 
 */
void Initializer::setKeyframe(cv::Mat keyframe) {
    keyframe_pyr = preprocessFrame(keyframe);
    last_pyr = keyframe_pyr;

    std::vector<cv::KeyPoint> features;
    FAST->detect(last_pyr[0], features);

    last_points.clear();
    last_points.reserve(features.size());

    for (auto feature : features) {
        last_points.push_back(feature.pt);
    }
}

/**
 * @brief Creates `cv::imshow()`-able image that shows our state.
 * 
 * @return cv::Mat 
 */
cv::Mat Initializer::createDebugImage() {
    cv::Mat out;

    // Add color channels so we can draw w/ color.
    cv::cvtColor(last_pyr[0], out, cv::COLOR_GRAY2BGR);

    for (auto pt : last_points) {
        int x = pt.x + 0.5;
        int y = pt.y + 0.5;
        cv::circle(out, cv::Point(x, y), 1.0, cv::Scalar(0, 0, 255), -1);
    }

    return out;
}

/**
 * @brief Transform frame into a preprocessed pyramid.
 * 
 * @param frame 
 * @return cv::Mat 
 */
std::vector<cv::Mat> Initializer::preprocessFrame(const cv::Mat& frame) {
    // Do some basic preprocesing on the original frame.
    cv::Mat frame_gray, frame_equalized;
    std::vector<cv::Mat> out;

    cv::cvtColor(frame, frame_gray, cv::COLOR_BGR2GRAY);
    CLAHE->apply(frame_gray, frame_equalized);

    cv::buildOpticalFlowPyramid(frame_equalized, out, cv::Size(9, 9), 4);

    return out;
}