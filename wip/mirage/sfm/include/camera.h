#ifndef MIRAGE_CAMERA_H
#define MIRAGE_CAMERA_H

#include <opencv2/opencv.hpp>

class Camera {
public:
    Camera(unsigned int camera_idx);

    cv::Mat nextFrame();

private:
    unsigned int camera_idx;
    cv::VideoCapture native_device;
};

#endif  // MIRAGE_CAMERA_H