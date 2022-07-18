#include <stdio.h>
#include <stdlib.h>
#include <chrono>

#include <opencv2/opencv.hpp>

#include "initializer.h"

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

    cv::Mat curframe;
    camera >> curframe;

    Initializer initializer(curframe);

    while (true) {
        // Exit if esc or q key is pressed
        int key = cv::waitKey(1);
        if (key == 'q' || key == 27 /* ESC */) {
            break;
        }

        camera >> curframe;

        std::vector<cv::Point3d> initial_points;
        initializer.attemptTriangulation(curframe, initial_points);

        cv::imshow("current_frame", curframe);
        cv::imshow("initialization", initializer.createDebugImage());
    }
}