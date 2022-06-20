/**
 * Estimates the fundamental matrix from a webcam stream.
 */

#include <stdio.h>

#include <chrono>
#include <set>
#include <vector>

#include <opencv2/opencv.hpp>
#include <opencv2/imgproc.hpp>
#include <opencv2/line_descriptor.hpp>

static cv::Ptr<cv::Feature2D> FAST = cv::FastFeatureDetector::create();
//static cv::Ptr<cv::Feature2D> BRIEF = cv::xfeatures2d::BriefDescriptorExtractor::create();
static cv::Ptr<cv::Feature2D> ORB = cv::ORB::create(100, 1.0, 0);  // opencv contrib not enabled, can't use non-ORB BRIEF :c
static cv::Ptr<cv::Feature2D> ORB2 = cv::ORB::create(
    500,                    // nfeatures
    1.3f,                   // scaleFactor
    10,                     // nlevels
    31,                     // edgeThreshold
    0,                      // firstLevel
    2,                      // WTA_K
    cv::ORB::HARRIS_SCORE,  // scoreType
    31,                     // patchSize
    20                      // fastThreshold
    );

void printMat(cv::Mat mat, const char* label = NULL) {
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

    // Process captures.
    std::vector<cv::KeyPoint> keypoints;
    cv::Mat keydescriptors;
    cv::Mat keyframe;

    while (true) {
        // Exit if esc or q key is pressed
        int key = cv::waitKey(1);
        if (key == 'q' || key == 27 /* ESC */) {
            break;
        }

        // Process next frame
        cv::Mat frame;
        camera >> frame;
        //cv::imshow("Capture", frame);

        // Register a new keyframe if there isn't already one.
        if (keypoints.size() == 0) {
            keyframe = frame.clone();
            ORB2->detect(keyframe, keypoints);
            ORB2->compute(keyframe, keypoints, keydescriptors);
            continue;
        }

        // Get frame feature points & descriptors.
        std::vector<cv::KeyPoint> points;
        cv::Mat descriptors;

        ORB2->detect(frame, points);
        if (points.size() == 0) {
            continue;
        }

        ORB2->compute(frame, points, descriptors);

        // Match frame points to keyframe points, attempt to filter false matches.
        std::vector<std::vector<cv::DMatch>> matches;

        cv::BFMatcher matcher = cv::BFMatcher(cv::NORM_L2);
	    matcher.knnMatch(descriptors, keydescriptors, matches, 2);

        std::vector<cv::DMatch> good_matches;
        for (int i=0; i < matches.size(); i++) {
            auto m = matches[i][0];
            auto n = matches[i][1];

            if (m.distance < 0.75 * n.distance) {
                good_matches.push_back(m);
            }
        }

        std::vector<cv::Point2f> p1(good_matches.size());
        std::vector<cv::Point2f> p2(good_matches.size());

        for (auto match : good_matches) {
            // For matches returned by knnMatch, queryIdx refers to first arg's
            // points and trainIdx refers to second arg's points.
            p1.push_back(points[match.queryIdx].pt);
            p2.push_back(keypoints[match.trainIdx].pt);
        }

        // NOTE: `F` is used almost universally in literature and research to
        //       denote the fundamental matrix, so despite the fact that this
        //       means I'm ever closer to writing code like an academic (*gag*)
        //       I'll use the same convention here since its likely familiar.
        cv::Mat F = cv::findFundamentalMat(p1, p2, cv::FM_RANSAC, 3.0, 0.99);
        printMat(F, "F");

        /*
        // Debug output
	    printf("%lu %lu %lu\n", points.size(), keypoints.size(), matches.size());
        if (matches.size() == 0) {
            keypoints.clear();
            continue;
        }
        */

        std::vector<cv::KeyPoint> good_points(p1.size());
        for (auto point : p1) {
            good_points.push_back(cv::KeyPoint(point, 1));
        }

	    cv::Mat out1;
        cv::drawKeypoints(frame, good_points, out1);
        cv::imshow("matching points", out1);


	    //cv::Mat out2;
	    //cv::drawMatches(frame, points, keyframe, keypoints, good_matches, out2, cv::Scalar(255, 0, 0), cv::Scalar(0, 255, 0));
        //cv::imshow("tracking", out2);

        keypoints.clear();
    }

    return 0;
}