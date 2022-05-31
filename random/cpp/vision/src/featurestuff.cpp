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
static cv::Ptr<cv::line_descriptor::LSDDetector> LINE_DETECTOR = cv::line_descriptor::LSDDetector::createLSDDetector();

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

void draw_lines(
    cv::Mat& image,
    const std::vector<cv::line_descriptor::KeyLine>& lines,
    cv::Scalar color
) {
    for (size_t i=0; i < lines.size(); i++) {
		auto line = lines[i];
		cv::Point pt1 = cv::Point(line.startPointX, line.startPointY);
		cv::Point pt2 = cv::Point(line.endPointX,   line.endPointY);
		cv::line(image, pt1, pt2, color, 1);
    }
}

int main(int argc, char** argv) {
    int camera_idx = 0;
    if (argc > 1)
        camera_idx = atoi(argv[1]);

    cv::VideoCapture camera(camera_idx);
    if (!camera.isOpened()) {
        printf("Can't open camera %u\n", camera_idx);
        return 0;
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

        // ORB
        std::vector<cv::KeyPoint> orb_keypoints;
        cv::Mat orb_descriptors;

        std::chrono::steady_clock::time_point begin = std::chrono::steady_clock::now();
		ORB->detectAndCompute(image, cv::noArray(), orb_keypoints,
                orb_descriptors, false);
        std::chrono::steady_clock::time_point end = std::chrono::steady_clock::now();
        
        PRINT_DEBUG("ORB time   = %.4f\n",
                (float)(std::chrono::duration_cast<std::chrono::milliseconds>(end - begin).count()) / 1000.0f);

        // AKAZE
        std::vector<cv::KeyPoint> akaze_keypoints;
        cv::Mat akaze_descriptors;

        begin = std::chrono::steady_clock::now();
		AKAZE->detectAndCompute(image, cv::noArray(), akaze_keypoints,
                akaze_descriptors, false);
        end = std::chrono::steady_clock::now();
        
        PRINT_DEBUG("AKAZE time = %.4f\n",
                (float)(std::chrono::duration_cast<std::chrono::milliseconds>(end - begin).count()) / 1000.0f);

        PRINT_DEBUG("len(AKAZE) = %u, len(ORB) = %u\n", akaze_keypoints.size(), orb_keypoints.size());

	    // Lines
        std::vector<cv::line_descriptor::KeyLine> lines;

        begin = std::chrono::steady_clock::now();
	    cv::Mat mask = cv::Mat::ones(image.size(), CV_8UC1);
	    LINE_DETECTOR->detect(image, lines, 2, 1, mask);
        end = std::chrono::steady_clock::now();

        PRINT_DEBUG("LSD time = %.4f\n",
                (float)(std::chrono::duration_cast<std::chrono::milliseconds>(end - begin).count()) / 1000.0f);
		PRINT_DEBUG("LSD #    = %lu\n", lines.size());

        // Draw debug output.
        draw_keypoints(image, akaze_keypoints, cv::Scalar(255, 0, 0), cv::Scalar(0, 255, 0));
        draw_keypoints(image, orb_keypoints,   cv::Scalar(0, 0, 255), cv::Scalar(0, 255, 255));
        draw_lines(image,     lines,           cv::Scalar(0, 255, 0));
        cv::imshow("features", image);
    }

    return 0;
}
