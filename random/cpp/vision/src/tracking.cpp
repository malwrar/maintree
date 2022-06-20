#include <stdio.h>

#include <chrono>
#include <set>
#include <vector>

#include <opencv2/opencv.hpp>
#include <opencv2/imgproc.hpp>
#include <opencv2/line_descriptor.hpp>

static cv::Ptr<cv::Feature2D> FAST = cv::FastFeatureDetector::create(10);
static cv::Ptr<cv::CLAHE> CLAHE = cv::createCLAHE();

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

class Capture {
public:
    Capture(const cv::Mat& image) {
        cv::Mat image_greyscale;
        cv::cvtColor(image, this->image, cv::COLOR_BGR2GRAY);
        //CLAHE->apply(image_greyscale, this->image);
    }

    std::vector<cv::KeyPoint> getEdges() {
        if (keypoints.size() == 0)
            FAST->detect(image, keypoints);

        return keypoints;
    }

    cv::Mat getImage() {
        return image.clone();
    }

private:
    cv::Mat image;
    std::vector<cv::KeyPoint> keypoints;
};


class Tracking {
public:
    Tracking() : keyframe(nullptr), curframe(nullptr) { }
 
    ~Tracking() {
	if (keyframe != nullptr) { delete keyframe; }
	if (curframe != nullptr) { delete curframe; }
    }

    /**
     * Process next webcam capture.
     */
    void submitImage(const cv::Mat& image) {
        assert(!image.empty() && image.channels() == 3);

        Capture* capture = new Capture(image);

        if (anchors.size() == 0) {
            keyframe = nullptr;
        }
	
	// Manage captures.
	if (keyframe == nullptr) {
            keyframe = capture;
            bootstrap();
            return;
        } else if (curframe != nullptr) {
            delete curframe;
        }

        curframe = capture;

        // Track keyframe features in current frame.
        std::vector<unsigned char> status;
        std::vector<float> error;
        std::vector<cv::Point2f> tracked_anchors(anchors.size());

        cv::calcOpticalFlowPyrLK(keyframe->getImage(), curframe->getImage(),
            anchors, tracked_anchors, status, error, cv::Size(41, 41), 4);

	// Prune any features that couldn't be tracked in the next frame.
        std::vector<cv::Point2f> good_anchors;
        cur_anchors.clear();

        for (int i=0; i < anchors.size(); i++) {
            if (status[i]) {
                good_anchors.push_back(anchors[i]);
                cur_anchors.push_back(tracked_anchors[i]);
            }
        }

        // Determine if tracking was successful
        if ((float)good_anchors.size() / (float)anchors.size() < 0.8) {
            printf("Lost tracking.");
            keyframe = nullptr;
            curframe = nullptr;
            return;
        }

        anchors = good_anchors;
    }

    /**
     * Draw current tracking info on the current image for debugging purposes.
     *
     * Ensure `image` patches the most recent image submitted via `submitImage()`!
     */
    void annotateImage(cv::Mat& image) {
        printf("current anchors: %u\n", cur_anchors.size());

        for (int i=0; i < cur_anchors.size(); i++) {
            int x = cur_anchors[i].x + 0.5;
            int y = cur_anchors[i].y + 0.5;

            cv::circle(image, cv::Point(x, y), 1.0, cv::Scalar(255, 0, 0), -1);
        }

        //if (anchors.size() == cur_anchors.size()) {
        //    cv::Mat F = cv::findFundamentalMat(anchors, cur_anchors, cv::FM_RANSAC, 4.0, 0.99);
        //    printMat(F, "F");
        //}
    }

private:
    Capture* keyframe;
    Capture* curframe;

    std::vector<cv::Point2f> anchors;
    std::vector<cv::Point2f> cur_anchors;

    void bootstrap() {
        assert(keyframe != nullptr);

        auto keypoints = keyframe->getEdges();

        anchors.clear();
        anchors = std::vector<cv::Point2f>(keypoints.size());

        for (auto keypoint : keypoints) {
            anchors.push_back(keypoint.pt);
        }
        printf("initial keypoints: %u\n", keypoints.size());
        printf("initial anchors:   %u\n", anchors.size());
    }
};

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

    Tracking tracker;

    while (true) {
        // Exit if esc or q key is pressed
        int key = cv::waitKey(1);
        if (key == 'q' || key == 27 /* ESC */) {
            break;
        }

        // Process next frame
        cv::Mat frame;
        camera >> frame;

        tracker.submitImage(frame);
        tracker.annotateImage(frame);

        cv::imshow("tracking", frame);
    }
}
