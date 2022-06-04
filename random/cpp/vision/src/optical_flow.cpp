#include <stdio.h>

#include <iostream>

#include <opencv2/opencv.hpp>
#include <opencv2/imgproc.hpp>

static cv::Ptr<cv::CLAHE> CLAHE = cv::createCLAHE();
static cv::Ptr<cv::Feature2D> AKAZE = cv::AKAZE::create();
static cv::Ptr<cv::Feature2D> ORB = cv::ORB::create();
static cv::Ptr<cv::Feature2D> FAST = cv::FastFeatureDetector::create();

std::vector<cv::Point2f> findFeatures(cv::Mat& image) {
    /*
    // Locate features using a few different detectors.
    std::vector<cv::KeyPoint> orb_features;
    ORB->detect(image, orb_features);

    std::vector<cv::KeyPoint> akaze_features;
    AKAZE->detect(image, akaze_features);

    std::vector<cv::KeyPoint> fast_features;
    FAST->detect(image, akaze_features);

    // Combine feature points into one vector.
    std::vector<cv::Point2f> features(orb_features.size()
            + akaze_features.size());
    int i = 0;

    // https://stackoverflow.com/a/64794991
    for (const auto& v : {
        std::move(orb_features),
        std::move(akaze_features)
    }) {
        for (auto feature : v) {
            features[i++] = feature.pt;
        }
    }

    // TODO: filter?
    */

    std::vector<cv::Point2f> features;
    cv::goodFeaturesToTrack(image, features, 100, 0.3, 7, cv::Mat(), 7, false, 0.04);

    return features;
}

cv::Mat preprocessImage(cv::Mat& image) {
    cv::Mat image_grayscale;
    cv::cvtColor(image, image_grayscale, cv::COLOR_BGR2GRAY);

    cv::Mat clahe_filtered;
    CLAHE->apply(image_grayscale, clahe_filtered);

    return clahe_filtered;
}

int main(int argc, char **argv) {
    int camera_idx = 0;
    if (argc > 1)
        camera_idx = atoi(argv[1]);

    cv::VideoCapture camera(camera_idx);
    if (!camera.isOpened()) {
        printf("Can't open camera %u\n", camera_idx);
        return 0;
    }

    // Fetch the first frame (our keyframe) + metadata.
    cv::Mat keyframe_orig;
    camera >> keyframe_orig;

    cv::Mat keyframe_processed = preprocessImage(keyframe_orig);
    std::vector<cv::Point2f> features = findFeatures(keyframe_processed);
    cv::imshow("keyframe", keyframe_processed);

    // Create some random colors to illustrate features.
    cv::RNG rng;
    std::vector<cv::Scalar> colors;
    for(int i=0; i < features.size(); i++) {
        int r = rng.uniform(0, 256);
        int g = rng.uniform(0, 256);
        int b = rng.uniform(0, 256);
        colors.push_back(cv::Scalar(r,g,b));
    }

    // Attempt to track features across subsequent frames using optical flow.
    // This is much faster than recalculating them for each frame.
    cv::Mat debug_mask = cv::Mat::zeros(
            keyframe_processed.size(),
            keyframe_processed.type());

    while (true) {
        // Exit if esc or q key is pressed
        int key = cv::waitKey(1);
        if (key == 'q' || key == 27 /* ESC */) {
            break;
        }

        // Read next frame..
        cv::Mat frame_orig;
        camera >> frame_orig;

        cv::Mat debug_image = frame_orig.clone();

        cv::Mat frame_processed = preprocessImage(frame_orig);
        cv::imshow("frame", frame_processed);

        // Relocate features.
        auto window_size = cv::Size(15, 15);
        auto num_pyramids = 4;
        auto termination_criteria = cv::TermCriteria(
                (cv::TermCriteria::COUNT) + (cv::TermCriteria::EPS),
                10,     //
                0.03);  //

        std::vector<cv::Point2f> relocated_features(features.size());
        std::vector<uchar> status(features.size());
        std::vector<float> err(features.size());

        std::chrono::steady_clock::time_point begin = std::chrono::steady_clock::now();

        calcOpticalFlowPyrLK(keyframe_processed, frame_processed,
                features, relocated_features, status, err,
                window_size, num_pyramids, termination_criteria);

        std::chrono::steady_clock::time_point end = std::chrono::steady_clock::now();
        printf("Flow time  = %.4f\n",
                (float)(std::chrono::duration_cast<std::chrono::milliseconds>(end - begin).count()) / 1000.0f);

        // TODO: calculate error, do stuff with it?

        // Remove points that can't be retracked.
        std::vector<cv::Point2f> filtered_features;
        for(int i=0; i < features.size(); i++) {
            // NOTE: `status` maps, for each feature in `features`, a
            //       1 if it has been relocated, or a 0 if it has not.
            if (status[i]) {
                auto feature = features[i];
                filtered_features.push_back(feature);

                // Draw path from keyframe point to relocated point.
                cv::arrowedLine(debug_image, relocated_features[i], features[i], colors[i], 1);

                // Draw relocated feature approximate location
                cv::circle(debug_image, relocated_features[i], 3, colors[i], 1);

                // Draw feature initial location
                // TODO: normalize error
                //cv::circle(debug_image, relocated_features[i], 2 * err[i], colors[i], -1);
            }
        }
        features = filtered_features;
        printf("features: %u\n", features.size());

        // Debug
        cv::imshow("tracking", debug_image);
    }
}
