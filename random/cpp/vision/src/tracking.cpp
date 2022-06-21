#include <stdio.h>

#include <chrono>
#include <set>
#include <vector>

#include <opencv2/opencv.hpp>
#include <opencv2/imgproc.hpp>
#include <opencv2/line_descriptor.hpp>

static cv::Ptr<cv::Feature2D> FAST = cv::FastFeatureDetector::create(10);
static cv::Ptr<cv::CLAHE> CLAHE = cv::createCLAHE();

namespace util {

    size_t count_nonzero(std::vector<unsigned char>& mask) {
        size_t count = 0;

        for (size_t i=0; i < mask.size(); i++) {
            if (!mask[i]) { continue; }

            count++;
	}

        return count;
    }

    template <typename T>
    void prune_vector(std::vector<T>& vec, std::vector<unsigned char>& mask) {
        assert(vec.size() == mask.size());

        std::vector<T> old = vec;

        vec.clear();
        vec.resize(mask.size());

        for (size_t i=0; i < mask.size(); i++) {
            if (!mask[i]) { continue; }

            vec.push_back(old[i]);
        }
    }

    void print_matrix(cv::Mat mat, const char* label = NULL) {
        if (label != NULL) {
            printf("%s = \n", label);
        }

        for (int row=0; row < mat.rows; row++) {
            printf("[ ");

            for (int col=0; col < mat.cols; col++) {
                float entry = mat.at<float>(row, col);
                printf("%.03f ", entry);
            }

            printf("]\n");
        }
    }

    void form_intrinsic_matrices(
        cv::Mat& K,
        cv::Mat& dist_coeff,
        float fx,
        float fy,
        float cx,
        float cy,
        float k1,
        float k2,
        float p1,
        float p2
    ) {
    
    }
}

/**
 *
 */
enum TrackerState {
    Bootstrapping,
    Initializing,
    Tracking,
};

/**
 * Given a continuous set of sequential, close captures from a single monocular
 * camera, this class will attempt to track the 6 DoF pose of the camera in
 * real time while simultaneously mapping the surrounding environment.
 *
 * Note that this class does not perform any offline refinement, and will
 * likely drift over large distances.
 */
class Tracker {
public:
    Tracker() : state(TrackerState::Bootstrapping) { }
 
    /**
     * Submit next image from camera for tracking.
     */
    bool submit(
        const cv::Mat& image,
        cv::Mat K,
        cv::Mat dist_coef
    ) {
        printf("features: %lu\n", last_points_2d.size());
        cv::Mat processed_image = enhanceImage(image);

        switch (state) {
        case TrackerState::Bootstrapping:
            return bootstrap(processed_image, K, dist_coef);
        case TrackerState::Initializing:
            return initialize(processed_image, K, dist_coef);
        case TrackerState::Tracking:
            return track(processed_image, K, dist_coef);
        default:
            return false;
	}
    }

    bool debug(
        cv::Mat& out_image,
	bool use_unprocessed_image=true
    ) {
	// Draw 2d keypoints.
        for (int i=0; i < last_points_2d.size(); i++) {
            auto point = last_points_2d[i];

            int x = point.x + 0.5;
            int y = point.y + 0.5;

            cv::circle(out_image, cv::Point(x, y), 1.0, cv::Scalar(255, 0, 0), -1);
        }

	return true;
    }

private:
    TrackerState state;

    cv::Mat last_image;
    std::vector<cv::KeyPoint> last_features;
    //std::vector<cv::KeyPoint> descriptors;
    std::vector<cv::Point2f> last_points_2d;

    /**
     *
     */
    bool bootstrap(
        const cv::Mat& image,
        cv::Mat K,
        cv::Mat dist_coef
    ) {
        reset();

	last_image = image;

        FAST->detect(last_image, last_features);

        last_points_2d.reserve(last_features.size());
        for (auto feature : last_features) {
            last_points_2d.push_back(feature.pt);
        }

        state = TrackerState::Initializing;

        return false;
    }

    /**
     *
     */
    bool initialize(
        const cv::Mat& image,
        cv::Mat K,
        cv::Mat dist_coef
    ) {
        // Forward optical flow tracking
        std::vector<unsigned char> status(last_features.size());
        std::vector<float> error(last_features.size());

        std::vector<cv::Point2f> retracked_points(last_features.size());

        cv::calcOpticalFlowPyrLK(last_image, image, last_points_2d,
                retracked_points, status, error, cv::Size(9, 9), 5);
                
        printf("Points eliminated in forward track:  %lu\n",
                status.size() - util::count_nonzero(status));

        std::vector<cv::Point2f> good_points;
        for (size_t i=0; i < retracked_points.size(); i++) {
            if (!status[i]) { continue; }
            good_points.push_back(retracked_points[i]);
        }

        // Backwards optical flow retracking.
        status.clear();
        error.clear();

        status.reserve(good_points.size());
        error.reserve(good_points.size());

        std::vector<cv::Point2f> backtracked_points(last_features.size());

        cv::calcOpticalFlowPyrLK(image, last_image, good_points,
                backtracked_points, status, error, cv::Size(9, 9), 1);

        printf("Points eliminated in backward track: %lu\n",
                status.size() - util::count_nonzero(status));

        std::vector<cv::Point2f> gooder_points;
        for (size_t i=0; i < good_points.size(); i++) {
            if (!status[i]) { continue; }
            gooder_points.push_back(good_points[i]);
        }

        last_points_2d = gooder_points;

        last_image = image;

        return false;
    }

    /**
     *
     */
    bool track(
        const cv::Mat& image,
        cv::Mat K,
        cv::Mat dist_coef
    ) {

        return true;
    }

    /**
     *
     */
    void reset() {
        state = TrackerState::Bootstrapping;
        last_features.clear();
        last_points_2d.clear();
    }

    /**
     *
     */
    cv::Mat enhanceImage(const cv::Mat& image) {
        // Do some basic preprocesing on the original image.
        cv::Mat image_gray, image_processed;

        cv::cvtColor(image, image_gray, cv::COLOR_BGR2GRAY);
        CLAHE->apply(image_gray, image_processed);

	return image_processed;
    }

};

int main(int argc, char **argv) {
    // Setup webcam.
    int camera_idx = 0;
    if (argc > 1)
        camera_idx = atoi(argv[1]);

    cv::VideoCapture camera(camera_idx);
    if (!camera.isOpened()) {
        printf("Can't open camera %u\n", camera_idx);
        return 0;
    }

    Tracker tracker;

    while (true) {
        // Exit if esc or q key is pressed
        int key = cv::waitKey(1);
        if (key == 'q' || key == 27 /* ESC */) {
            break;
        }

        // TODO: do camera calibration, or take it from config/args
        cv::Mat K, dist_coef;

        // Process next frame
        cv::Mat frame;
        camera >> frame;

        tracker.submit(frame, K, dist_coef);

        if (tracker.debug(frame))
            cv::imshow("tracking", frame);
    }
}
