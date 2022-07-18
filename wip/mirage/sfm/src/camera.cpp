#include "camera.h"

Camera::Camera(unsigned int camera_idx) : camera_idx(camera_idx) {
    native_device = cv::VideoCapture(camera_idx);
    if (!camera.isOpened()) {
        printf("Can't open camera %u\n", camera_idx);
        throw "Failed to open camera.";
    }
}

cv::Mat Camera::nextFrame() {
    cv::Mat image;
    camera >> image;

    return image;
}
