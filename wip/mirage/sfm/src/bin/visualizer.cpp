#include <stdio.h>
#include <stdlib.h>
#include <chrono>

#include <opencv2/opencv.hpp>

#include "initializer.h"
#include "util.h"

int main(int argc, char* argv[]) {
    // Setup webcam.
    int camera_idx = 0;
    if (argc > 1)
        camera_idx = atoi(argv[1]);

    cv::VideoCapture camera(camera_idx);
    if (!camera.isOpened()) {
        printf("Can't open camera %u\n", camera_idx);
        return 0;
    }

    // TODO: do camera calibration, or take it from config/args
    cv::Mat K, dist_coef;
    util::form_intrinsic_matrices(K, dist_coef, 
        644.399443271804,
        644.9971064933079,
        336.3097152007661,
        242.1983578514321,
        -0.4709080550613399,
        0.6048183201946264,
        -0.0065762106050252685,
        0.0014721495567369194);

    cv::Mat curframe;
    camera >> curframe;

    Initializer initializer(curframe, K, dist_coef);

    while (true) {
        // Exit if esc or q key is pressed
        int key = cv::waitKey(1);
        if (key == 'q' || key == 27 /* ESC */) {
            break;
        }

        camera >> curframe;

        std::vector<cv::Point3f> initial_points;
        cv::Mat Rcw, Tcw;
        if (initializer.attemptTriangulation(curframe, Rcw, Tcw, initial_points)) {
            printf("!!!!! Triangulated %lu!\n", initial_points.size());
        }

        cv::imshow("current_frame", curframe);
        cv::imshow("initialization", initializer.createDebugImage());
    }
}