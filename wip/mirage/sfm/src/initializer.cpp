#include "initializer.h"
#include "triangulation.h"
#include "util.h"

static cv::Ptr<cv::CLAHE> CLAHE = cv::createCLAHE();
static cv::Ptr<cv::Feature2D> FAST = cv::FastFeatureDetector::create(10);

bool Initializer::attemptTriangulation(
    cv::Mat frame,
    cv::Mat& Rcw,
    cv::Mat& Tcw, 
    std::vector<cv::Point3f>& p3d
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

    util::prune_vector(keyframe_points, status);
    util::prune_vector(retracked_points, status);

    // TODO: backwards track & prune any that are untracked or some # of pixels off
    // TODO: perform filtering

    // Estimate distance between points and attempt triangulation if parallax
    // is sufficient.
    bool ret = false;

    cv::Mat transform = cv::estimateAffinePartial2D(keyframe_points, retracked_points);
    printf("cv::norm(transform.col(2)) = %lf\n", cv::norm(transform.col(2)));

    if (cv::norm(transform.col(2)) > 100) {
        printf("Attempting triangulation\n");
        ret = triangulate(keyframe_points, retracked_points, Rcw, Tcw, p3d);
    }

    // Get ready for the next invocation.
    last_pyr = pyramid;
    last_points = retracked_points;

    return ret;
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
    keyframe_points.clear();

    keyframe_points.reserve(features.size());
    keyframe_points.reserve(features.size());

    for (auto feature : features) {
        keyframe_points.push_back(feature.pt);
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

bool Initializer::triangulate(
    std::vector<cv::Point2f> p1,
    std::vector<cv::Point2f> p2,
    cv::Mat& Rcw,
    cv::Mat& Tcw,
    std::vector<cv::Point3f>& p3
) {
    std::vector<unsigned char> status(p1.size());

    cv::Mat F = cv::findFundamentalMat(p1, p2, cv::FM_RANSAC, 3, 0.99, status);
    if (F.empty()) {
        printf("Failed to find fundamental matrix.\n");
        return false;
    }
    util::prune_vector(p1, status);
    util::prune_vector(p2, status);

    return math::triangulateF(p1, p2, F, K, Rcw, Tcw, p3);

    /*

    cv::Mat F = cv::findFundamentalMat(keyframe_points, retracked_points, cv::FM_RANSAC, 3, 0.99, status);
    if (!F.empty()) {
        util::prune_vector(keyframe_points, status);
        util::prune_vector(retracked_points, status);

        printf("Points eliminated by fundamental matrix:      %lu\n",
                status.size() - util::count_nonzero(status));

        cv::Mat E = K.t() * F * K;
        if (fabsf(cv::determinant(E)) <= 1e-07) {
            printf("det(E) = %f\n", cv::determinant(E));

            std::vector<cv::Point2f> p1, p2;

            cv::undistortPoints(keyframe_points,  p1, K, cv::Mat());
            cv::undistortPoints(retracked_points, p2, K, cv::Mat());

            if (util::triangulate(p1, p2, points_3d, status, E)) {
                printf("aaa %lu %lu\n", keyframe_points.size(), status.size());
                util::prune_vector(keyframe_points, status);
                util::prune_vector(retracked_points, status);
                util::prune_vector(points_3d, status);

                for (size_t i=0; i < points_3d.size(); i++) {
                    auto point = points_3d[i];

                    printf("%03u: [%lf, %lf, %lf]\n", i, point.x, point.y, point.z);
                }

                // Get ready for the next invocation.
                last_pyr = pyramid;
                last_points = retracked_points;

                return true;
            }
        }
    }
    */
}