#include <stdio.h>

#include <chrono>
#include <set>
#include <vector>

#include <opencv2/opencv.hpp>
#include <opencv2/imgproc.hpp>
#include <opencv2/line_descriptor.hpp>

static cv::Ptr<cv::Feature2D> AKAZE = cv::AKAZE::create();
static cv::Ptr<cv::Feature2D> ORB = cv::ORB::create();
static cv::Ptr<cv::CLAHE> CLAHE = cv::createCLAHE();

#define DEBUG
#ifdef DEBUG
#define PRINT_DEBUG(...) do{ fprintf( stderr, __VA_ARGS__ ); } while( false )
#else
#define PRINT_DEBUG(...) do{ } while ( false )
#endif

void draw_keypoints(
    cv::Mat& image,
    const std::vector<cv::KeyPoint>& keypoints,
    cv::Scalar point_color,
    cv::Scalar circle_color
) {
    for (size_t i=0; i < keypoints.size(); i++) {
        int x = keypoints[i].pt.x + 0.5;
        int y = keypoints[i].pt.y + 0.5;
        float radius = keypoints[i].size / 2.0;

        //cv::circle(image, cv::Point(x, y), 2.5 * radius, circle_color, 1);
        cv::circle(image, cv::Point(x, y), 1.0, point_color, -1);
    }
}

class Frame {
public:
    Frame(cv::Mat image): image(image) {
        this->findFeatures();
    }

    void draw() {
        cv::Mat output = image.clone();
        draw_keypoints(output, akaze_features, cv::Scalar(255, 0, 0), cv::Scalar(0, 255, 0));
        draw_keypoints(output, orb_features,   cv::Scalar(0, 0, 255), cv::Scalar(0, 255, 255));
        cv::imshow("frame", output);       
    }

private:
    void findFeatures() {
        // ORB
        std::chrono::steady_clock::time_point begin = std::chrono::steady_clock::now();
        ORB->detect(image, orb_features);
        std::chrono::steady_clock::time_point end = std::chrono::steady_clock::now();
        PRINT_DEBUG("ORB time   = %.4f\n",
                (float)(std::chrono::duration_cast<std::chrono::milliseconds>(end - begin).count()) / 1000.0f);

        // AKAZE
        begin = std::chrono::steady_clock::now();
        AKAZE->detect(image, akaze_features);
        end = std::chrono::steady_clock::now();
        PRINT_DEBUG("AKAZE time = %.4f\n",
                (float)(std::chrono::duration_cast<std::chrono::milliseconds>(end - begin).count()) / 1000.0f);
    }

    cv::Mat image;

    std::vector<cv::KeyPoint> orb_features;
    std::vector<cv::KeyPoint> akaze_features;
};

int main(int argc, char** argv) {
    int camera_idx = 0;
    if (argc > 1)
        camera_idx = atoi(argv[1]);

    cv::VideoCapture camera(camera_idx);
    if (!camera.isOpened()) {
        printf("Can't open camera %u\n", camera_idx);
        return 0;
    }

    Frame* cur_frame = nullptr;
    Frame* last_frame = nullptr;

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

        cur_frame = new Frame(image);
        cur_frame->draw();

        if (last_frame != nullptr) {
            delete last_frame;
            last_frame = cur_frame;
        }
    }

    if (cur_frame != nullptr)
        delete cur_frame;
    if (last_frame != nullptr)
        delete last_frame;

    return 0;
}
