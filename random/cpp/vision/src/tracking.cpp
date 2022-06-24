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

    size_t count_nonzero(cv::Mat& mask) {
        size_t count = 0;

        for (size_t i=0; i < mask.rows; i++) {
            for (size_t j=0; j < mask.cols; j++) {
                if (!mask.at<unsigned char>(i, j)) { continue; }

                count++;
	    }
	}

        return count;
    }

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
        cv::Mat& dist_coef,
        double fx,
        double fy,
        double cx,
        double cy,
        double k1,
        double k2,
        double p1,
        double p2
    ) {
        K = cv::Mat::eye(3, 3, CV_64F);
        K.at<double>(0, 0) = fx;
        K.at<double>(1, 1) = fy;
        K.at<double>(0, 2) = cx;
        K.at<double>(1, 2) = cy;

        dist_coef = cv::Mat(4, 1, CV_64F);
        dist_coef.at<double>(0) = k1;
        dist_coef.at<double>(1) = k2;
        dist_coef.at<double>(2) = p1;
        dist_coef.at<double>(3) = p2;
    }
}

/**
 *
 */
enum TrackerState {
    Bootstrapping,
    Localizing,
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
        case TrackerState::Localizing:
            return localize(processed_image, K, dist_coef);
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
    std::vector<cv::Point2f> last_points_2d;
    //std::vector<cv::Vec3> last_points_3d;

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

        std::vector<cv::KeyPoint> features;
        FAST->detect(last_image, features);

        last_points_2d.reserve(features.size());
        for (auto feature : features) {
            last_points_2d.push_back(feature.pt);
        }

        state = TrackerState::Localizing;

        return false;
    }

    /**
     *
     */
    bool localize(
        const cv::Mat& image,
        cv::Mat K,
        cv::Mat dist_coef
    ) {
        if (last_points_2d.size() < 10) {
            state = TrackerState::Bootstrapping;
            return false;
        }

        // Forward optical flow tracking
        std::vector<unsigned char> status(last_points_2d.size());
        std::vector<float> error(last_points_2d.size());

        std::vector<cv::Point2f> retracked_points(last_points_2d.size());

        auto begin = std::chrono::steady_clock::now();

        cv::calcOpticalFlowPyrLK(last_image, image, last_points_2d,
                retracked_points, status, error, cv::Size(9, 9), 5);

        auto end = std::chrono::steady_clock::now();
	//printf("forward tracking    %.3f\n", (float)std::chrono::duration_cast<std::chrono::microseconds>(end - begin).count() / 1000000.0);

        util::prune_vector(last_points_2d, status);
        util::prune_vector(retracked_points, status);

        //printf("Points eliminated in forward optical flow:    %lu\n",
        //        status.size() - util::count_nonzero(status));

        // Backwards optical flow retracking.
        status.clear();
        error.clear();

        status.reserve(retracked_points.size());
        error.reserve(retracked_points.size());

        std::vector<cv::Point2f> backtracked_points(retracked_points.size());

        begin = std::chrono::steady_clock::now();

        cv::calcOpticalFlowPyrLK(image, last_image, retracked_points,
                backtracked_points, status, error, cv::Size(9, 9), 1);

        end = std::chrono::steady_clock::now();
	//printf("backward tracking   %.3f\n", (float)std::chrono::duration_cast<std::chrono::microseconds>(end - begin).count() / 1000000.0);

        util::prune_vector(last_points_2d, status);
        util::prune_vector(retracked_points, status);

        //printf("Points eliminated by backward optical flow:   %lu\n",
        //        status.size() - util::count_nonzero(status));

        // Attempt to verify features by looking for homographies
        //std::vector<std::vector<cv::Point2f>> local_homographies;
        //std::vector<cv::Point2f> ;

        if (last_points_2d.size() < 4) {
            state = TrackerState::Bootstrapping;
            return false;
        }

        begin = std::chrono::steady_clock::now();

        cv::Mat H, mask;
        H = cv::findHomography(last_points_2d, retracked_points, cv::RANSAC, 3.0, mask);

        //while (true) {
        //    if (last_points_2d.size() <= 44) { break; }

        //    cv::Mat H, mask;
        //    H = cv::findHomography(last_points_2d, retracked_points, cv::RANSAC, 3.0, mask);
        //    printf("a %lu\n", util::count_nonzero(mask));

        //    for (size_t i=mask.rows; i > 0; i--) {
        //        if (mask.at<unsigned char>(i, 0) == 0) {
        //            last_points_2d.erase(last_points_2d.begin() + (i - 1));
        //            retracked_points.erase(retracked_points.begin() + (i - 1));
        //        } else {
        //            p1.push_back(last_points_2d[i-1]);
        //            p2.push_back(retracked_points[i-1]);
        //        }
        //    }
        //}

        end = std::chrono::steady_clock::now();
	//printf("homography tracking %.3f\n", (float)std::chrono::duration_cast<std::chrono::microseconds>(end - begin).count() / 1000000.0);

        util::prune_vector(last_points_2d, mask);
        util::prune_vector(retracked_points, mask);

        if (last_points_2d.size() < 10) {
            state = TrackerState::Bootstrapping;
            return false;
        }

        //printf("Points eliminated by homography verification: %lu\n",
        //        mask.rows - util::count_nonzero(mask));

        // asdf
        cv::Mat rigidT = cv::estimateAffinePartial2D(last_points_2d, retracked_points);
        if (cv::norm(rigidT.col(2)) > 100) {
            printf("Attempting triangulation\n");

            // Attempt triangulation
            status.clear();
            status.reserve(last_points_2d.size());

            begin = std::chrono::steady_clock::now();

            cv::Mat F = cv::findFundamentalMat(last_points_2d, retracked_points, cv::FM_RANSAC, 3, 0.99, status);
            end = std::chrono::steady_clock::now();

            printf("fundamental matrix  %.3f\n", (float)std::chrono::duration_cast<std::chrono::microseconds>(end - begin).count() / 1000000.0);
	    if (!F.empty()) {
                util::prune_vector(last_points_2d, status);
                util::prune_vector(retracked_points, status);

                printf("Points eliminated by fundamental matrix:      %lu\n",
                        status.size() - util::count_nonzero(status));

                cv::Mat E = K.t() * F * K;
                if (fabsf(cv::determinant(E)) <= 1e-07) {
                    printf("det(E) = %f\n", cv::determinant(E));

                //    //cv::Mat_<double> R1(3, 3);
                //    //cv::Mat_<double> R2(3, 3);
                //    //cv::Mat_<double> t1(1, 3);
                //    //cv::Mat_<double> t2(1, 3);

                //    //if (decomposeE(E, R1, R2, t1, t2)) {

                //    //}
                }
	    }
        }

        // Save last 
        last_points_2d = retracked_points;
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

    /**
     *
     */
    void relocateFeatures() {

    }

    /**
     *
     */
    void filterNonHomographies(std::vector<cv::Point2f>& p1, std::vector<cv::Point2f>& p2) {

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
        util::form_intrinsic_matrices(K, dist_coef, 
            929.485616777751,
            819.5918436401257,
            243.95961493171143,
            339.8639601176542,
            0.6819743796586462,
            -34.53253298643073,
            -0.20280419786726658,
            0.09206263406958995);


        // Process next frame
        cv::Mat frame;
        camera >> frame;

        tracker.submit(frame, K, dist_coef);

        if (tracker.debug(frame))
            cv::imshow("tracking", frame);
    }
}
