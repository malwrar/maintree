#include <stdio.h>

#include <chrono>
#include <set>
#include <vector>

#include <opencv2/opencv.hpp>
#include <opencv2/imgproc.hpp>

static cv::Ptr<cv::Feature2D> AKAZE = cv::AKAZE::create();
static cv::Ptr<cv::CLAHE> CLAHE = cv::createCLAHE();

#define DEBUG
#ifdef DEBUG
#define PRINT_DEBUG(...) do{ fprintf( stderr, __VA_ARGS__ ); } while( false )
#else
#define PRINT_DEBUG(...) do{ } while ( false )
#endif

//class Keypoint {
//public:
//    Keypoint(int x, int y): 2d_x(x), 2d_y(y) {}
//
//private:
//    int 2d_x, 2d_y;
//}

void draw_keypoints(
    cv::Mat& image,
    const std::vector<cv::KeyPoint>& keypoints
) {
    for (size_t i=0; i < keypoints.size(); i++) {
        int x = keypoints[i].pt.x + 0.5;
        int y = keypoints[i].pt.y + 0.5;
	    float radius = keypoints[i].size / 2.0;

	    cv::circle(image, cv::Point(x, y), 2.5 * radius, cv::Scalar(0, 255, 0), 1);
	    cv::circle(image, cv::Point(x, y), 1.0, cv::Scalar(0, 0, 255), -1);
    }
}

int main(int argc, char** argv) {
    int camera_idx = 0;
    cv::VideoCapture camera(camera_idx);
    if (!camera.isOpened()) {
        printf("Can't open camera %u\n", camera_idx);
        return 1;
    }

    while (true) {
        // Exit if esc or q key is pressed
	    int key = cv::waitKey(1);
	    if (key == 'q' || key == 27 /* ESC */) {
            break;
	    }

        cv::Mat image;
        camera >> image;  // I really hate this syntax

        //cv::Mat image_contrast_enhanced;
	    //printf("%u %u\n", image.type(), image_contrast_enhanced.type());
	    //CLAHE->apply(image, image_contrast_enhanced);

        auto start = std::chrono::steady_clock::now();

        std::chrono::steady_clock::time_point begin = std::chrono::steady_clock::now();

	    std::vector<cv::KeyPoint> keypoints;
        cv::Mat descriptors;
	    AKAZE->detectAndCompute(image, cv::noArray(), keypoints, descriptors,
                    false);

        std::chrono::steady_clock::time_point end = std::chrono::steady_clock::now();
        
        PRINT_DEBUG("feature detection time = %.4f\n",
                (float)(std::chrono::duration_cast<std::chrono::milliseconds>(end - begin).count()) / 1000.0f);

	    draw_keypoints(image, keypoints);
        cv::imshow("webcam", image);
    }

    return 0;
}
