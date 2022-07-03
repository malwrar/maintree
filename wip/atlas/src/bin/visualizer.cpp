#include <opencv2/opencv.hpp>

#include "atlas.h"

static cv::Ptr<cv::Feature2D> AKAZE = cv::AKAZE::create();
static cv::Ptr<cv::Feature2D> FAST = cv::FastFeatureDetector::create(10);
static cv::Ptr<cv::CLAHE> CLAHE = cv::createCLAHE();

void draw_points(
    cv::Mat& image,
    const std::vector<cv::KeyPoint>& keypoints,
    cv::Scalar point_color
) {
    for (size_t i=0; i < keypoints.size(); i++) {
        int x = keypoints[i].pt.x + 0.5;
        int y = keypoints[i].pt.y + 0.5;
        float radius = keypoints[i].size / 2.0;

        //cv::circle(image, cv::Point(x, y), 2.5 * radius, circle_color, 1);
        cv::circle(image, cv::Point(x, y), 1.0, point_color, -1);
    }
}

int main(int argc, char* argv[]) {
    int camera_idx = 0;
    if (argc > 1)
        camera_idx = atoi(argv[1]);

    cv::VideoCapture camera(camera_idx);
    if (!camera.isOpened()) {
        printf("Can't open camera %u\n", camera_idx);
        return 0;
    }

    a();

    while (true) {
        // Stop if exit key pressed.
        int key = cv::waitKey(1);
        if (key == 'q' || key == 27 /* ESC */) {
            break;
        }

        cv::Mat image;
        camera >> image;  // I really hate this syntax

        cv::Mat image_gray, image_clahe, image_processed;

        auto enhance_begin = std::chrono::steady_clock::now();
        cv::cvtColor(image, image_gray, cv::COLOR_BGR2GRAY);
        CLAHE->apply(image_gray, image_processed);
        //CLAHE->apply(image_gray, image_clahe);
        //cv::fastNlMeansDenoising(image_clahe, image_processed, 10);
        auto enhance_end = std::chrono::steady_clock::now();

        cv::imshow("clahe", image_processed);

        std::vector<cv::KeyPoint> kp;
        cv::Mat kp_d;

        auto akaze_begin = std::chrono::steady_clock::now();
        AKAZE->detectAndCompute(image_processed, cv::noArray(), kp, kp_d);
        auto akaze_end = std::chrono::steady_clock::now();

        std::vector<cv::KeyPoint> kp2;
        cv::Mat kp_d2;

        AKAZE->detectAndCompute(image, cv::noArray(), kp2, kp_d2);

        std::vector<cv::KeyPoint> p;

        auto fast_begin = std::chrono::steady_clock::now();
        FAST->detect(image_processed, p);
        auto fast_end = std::chrono::steady_clock::now();

        cv::Mat debug_image = image.clone();

        cv::drawKeypoints(image, kp2, debug_image,
                cv::Scalar(0, 200, 200),
                cv::DrawMatchesFlags::DRAW_RICH_KEYPOINTS);

        draw_points(debug_image, kp, cv::Scalar(0, 150, 0));
        draw_points(debug_image, p, cv::Scalar(100, 0, 0));

        // Output timing info
        const size_t text_offset = 15;
        size_t y = text_offset;

        cv::putText(debug_image,
            (std::string("Enhance time: ")
                + std::to_string(
                    (float)(
                        std::chrono::duration_cast<std::chrono::milliseconds>(
                            enhance_begin - enhance_end
                        ).count()
                    )
                    / 1000.0f)
                + "ms").c_str(),
            cv::Point(2, y),
            cv::FONT_HERSHEY_PLAIN,
            1.0,
            CV_RGB(200, 200, 0));

        y += text_offset;

        cv::putText(debug_image,
            (std::string("AKAZE time: ")
                + std::to_string(
                    (float)(
                        std::chrono::duration_cast<std::chrono::milliseconds>(
                            akaze_end - akaze_begin
                        ).count()
                    )
                    / 1000.0f)
                + "ms").c_str(),
            cv::Point(2, y),
            cv::FONT_HERSHEY_PLAIN,
            1.0,
            CV_RGB(0, 200, 0));

        y += text_offset;

        cv::putText(debug_image,
            (std::string("FAST time: ")
                + std::to_string(
                    (float)(
                        std::chrono::duration_cast<std::chrono::milliseconds>(
                            fast_end - fast_begin
                        ).count()
                    )
                    / 1000.0f)
                + "ms").c_str(),
            cv::Point(2, y),
            cv::FONT_HERSHEY_PLAIN,
            1.0,
            CV_RGB(0, 0, 200));

        y += text_offset;

        cv::imshow("features", debug_image);
    }
}