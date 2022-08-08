#include <stdio.h>

#include <iostream>

#include <opencv2/opencv.hpp>
#include <opencv2/imgproc.hpp>

static cv::Ptr<cv::CLAHE> CLAHE = cv::createCLAHE();
static cv::Ptr<cv::Feature2D> FAST = cv::FastFeatureDetector::create();

void draw_keypoints(
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

class Capture {
public: 
    cv::Mat image_original;
    cv::Mat image_grayscale;
    cv::Mat image_enhanced;
    std::vector<cv::KeyPoint> features;

    Capture(cv::Mat& image) {
        this->image_original = image.clone();

        cv::cvtColor(this->image_original, this->image_grayscale,
                cv::COLOR_BGR2GRAY);

        CLAHE->apply(this->image_grayscale, this->image_enhanced);
    }

    std::vector<cv::KeyPoint> getFeatures() {
        if (this->features.size() == 0) {
            this->calcFeatures();
        }

        return this->features;
    }

    std::vector<cv::Point2f> getFeaturePoints() {
        auto features = this->getFeatures();

        std::vector<cv::Point2f> points(features.size());
        for (auto feature : features) {
            points.push_back(feature.pt);
        }

        return points;
    }

private:
    void calcFeatures() {
        FAST->detect(this->image_enhanced, this->features);
    }
};

struct Pose {

};

/**
 * @brief Attempts to track a single camera's pose given a series of captures.
 * 
 * WARN: This tracker makes a number of important assumptions:
 * 
 *       1.) Captures are real-time and in order.
 *       2.) Physical camera translation is varied.
 *       3.) Physical camera rotation is *gentle*.
 * 
 *       Tracker performance will degrade if these assumptions aren't met.
 * 
 * TODO: Augment tracker captures with depth (stereo, rgbd) and/or spatial
 *       (IMU, GPS) information if available, consider creating a `Capture`
 *       class.
 * 
 * This tracker, given a series of captures, attempts to efficienly identify
 * and relocate visual features across captures using a combination of optical
 * flow and homography calculation (???). After a sufficient number of
 * homographies (???) are established, the tracker will attempt to estimate the
 * 6DoF pose of the camera as well as the 3d positions of the relocated features.
 * 
 * In practical terms, this means that you'll need a submit minimum of 5 captures 
 * using `addCapture()` to initialize the tracker. After a sufficient number of
 * captures have been processed, the camera's estimated pose can be obtained
 * using `getCurrentPose()`. To avoid throwing exceptions, be sure to check the
 * state of the tracker using `getState()`.
 * 
 * TODO: search for previously encountered features and optimize poses by
 *       linking a map to the tracker using `linkLocalMap()`. Local maps should
 *       do some amount of online local processing and submit to a global map
 *       that performs slower offline optimizations.
 */
class Tracker {
public:
    enum State {
        INITIALIZING,  // Tracker needs more captures before
        TRACKING,      // Tracker has enough captures to calculate pose
                       // estimates.
    };

    Tracker() {
        this->state = State::INITIALIZING;
    }

    void addCapture(cv::Mat& image) {
        cv::imshow("addCapture(image)", image);
        this->addCapture(new Capture(image));

    }

    void addCapture(Capture* capture) {
        if (this->keyframe == nullptr) {
            this->keyframe = capture;
            return;
        }

        // Prepare for optical flow tracking
        const auto window_size = cv::Size(9, 9);
        const auto num_pyramids = 4;
        const auto termination_criteria = cv::TermCriteria(
                (cv::TermCriteria::COUNT) + (cv::TermCriteria::EPS),
                10,     //
                0.03);  //


        // Perform forward optical tracking.
        std::vector<cv::Point2f> start_points = keyframe->getFeaturePoints();
        std::vector<cv::Point2f> relocated_points(start_points.size());

        std::vector<uchar> fwd_status(start_points.size());
        std::vector<float> fwd_err(start_points.size());

        calcOpticalFlowPyrLK(this->keyframe.image_enhanced,
                capture->image_enhanced, start_points, relocated_points,
                fwd_status, fwd_err, window_size, num_pyramids-1,
                termination_criteria, (cv::OPTFLOW_USE_INITIAL_FLOW
                    + cv::OPTFLOW_LK_GET_MIN_EIGENVALS));

        start_points.clear();

        std::vector<cv::Point2f> fwd_points;
        for (int i=0; i < relocated_points.size(); i++) {
            if (status[i]) {
                fwd_points.push_back(relocated_points[i]);
            }
        }

        // Perform backwards optical tracking.
        std::vector<cv::Point2f> bwd_points(fwd_points.size());

        calcOpticalFlowPyrLK(capture->image_enhanced,
                this->keyframe.image_enhanced, fwd_points, bwd_points,
                status, error, window_size, 0, termination_criteria,
                (cv::OPTFLOW_USE_INITIAL_FLOW
                    + cv::OPTFLOW_LK_GET_MIN_EIGENVALS));

        std::vector<cv::Point2f> final_points;
        for (int i=0; i < relocated_points.size(); i++) {
            if (status[i]) {
                fwd_points.push_back(relocated_points[i]);
            }
        }
    }

    State getState() {
        return this->state;
    }

    Pose getCurrentPose() {
        return Pose { };
    }

    //void imshow() {
    //    cv::imshow("addCapture(capture)", capture.image_enhanced);
    //}

private:
    State state;
    Capture* keyframe;
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

    // Setup camera tracking.
    Tracker tracker;

    // Process captures.
    while (true) {
        // Exit if esc or q key is pressed
        int key = cv::waitKey(1);
        if (key == 'q' || key == 27 /* ESC */) {
            break;
        }

        // Process next frame
        cv::Mat frame;
        camera >> frame;

        Capture* capture = new Capture(frame);
        tracker.addCapture(capture);

        auto features = capture->getFeatures();
        delete capture;

        // Display image
        draw_keypoints(frame, features, cv::Scalar(0, 0, 255));

        cv::imshow("tracking", frame);
    }

    //// Fetch the first frame (our keyframe) + metadata.
    //cv::Mat keyframe_orig;
    //camera >> keyframe_orig;

    //cv::Mat keyframe_gray;
    //cv::cvtColor(keyframe_orig, keyframe_gray, cv::COLOR_BGR2GRAY);

    //cv::Mat keyframe_processed = preprocessImage(keyframe_orig);
    //std::vector<cv::Point2f> features = findFeatures(keyframe_processed);
    //cv::imshow("keyframe", keyframe_processed);

    //// Create some random colors to illustrate features.
    //cv::RNG rng;
    //std::vector<cv::Scalar> colors;
    //for(int i=0; i < features.size(); i++) {
    //    int r = rng.uniform(0, 256);
    //    int g = rng.uniform(0, 256);
    //    int b = rng.uniform(0, 256);
    //    colors.push_back(cv::Scalar(r,g,b));
    //}

    //// Attempt to track features across subsequent frames using optical flow.
    //// This is much faster than recalculating them for each frame.
    //cv::Mat debug_mask = cv::Mat::zeros(
    //        keyframe_processed.size(),
    //        keyframe_processed.type());

    //// blue moon hostel
    //while (true) {
    //    // Exit if esc or q key is pressed
    //    int key = cv::waitKey(1);
    //    if (key == 'q' || key == 27 /* ESC */) {
    //        break;
    //    }

    //    // Read next frame..
    //    cv::Mat frame_orig;
    //    camera >> frame_orig;

    //    cv::Mat debug_sparse = frame_orig.clone();

    //    cv::Mat frame_gray;
    //    cv::cvtColor(frame_orig, frame_gray, cv::COLOR_BGR2GRAY);

    //    cv::Mat frame_processed = preprocessImage(frame_orig);
    //    cv::imshow("frame", frame_processed);

    //    // Relocate features.
    //    const auto window_size = cv::Size(9, 9);
    //    const auto num_pyramids = 4;
    //    const auto termination_criteria = cv::TermCriteria(
    //            (cv::TermCriteria::COUNT) + (cv::TermCriteria::EPS),
    //            10,     //
    //            0.03);  //

    //    std::vector<cv::Point2f> relocated_features(features.size());
    //    std::vector<uchar> status(features.size());
    //    std::vector<float> err(features.size());

    //    std::chrono::steady_clock::time_point begin = std::chrono::steady_clock::now();

    //    calcOpticalFlowPyrLK(keyframe_processed, frame_processed,
    //            features, relocated_features, status, err,
    //            window_size, num_pyramids-1, termination_criteria);

    //    std::chrono::steady_clock::time_point end = std::chrono::steady_clock::now();
    //    printf("Lucas-Kanade Flow time = %.4f\n",
    //            (float)(std::chrono::duration_cast<std::chrono::milliseconds>(end - begin).count()) / 1000.0f);

    //    // TODO: calculate error, do stuff with it?

    //    // Remove points that can't be retracked.
    //    std::vector<cv::Point2f> filtered_features;
    //    for (int i=0; i < features.size(); i++) {
    //        // NOTE: `status` maps, for each feature in `features`, a
    //        //       1 if it has been relocated, or a 0 if it has not.
    //        if (status[i]) {
    //            auto feature = features[i];
    //            filtered_features.push_back(feature);

    //            // Draw path from keyframe point to relocated point.
    //            cv::arrowedLine(debug_sparse, relocated_features[i], features[i], colors[i], 1);

    //            // Draw relocated feature approximate location
    //            cv::circle(debug_sparse, relocated_features[i], 3, colors[i], 1);

    //            // Draw feature initial location
    //            // TODO: normalize error
    //            //cv::circle(debug_sparse, relocated_features[i], 2 * err[i], colors[i], -1);
    //        }
    //    }
    //    features = filtered_features;
    //    printf("features: %u\n", features.size());

    //    // Debug
    //    cv::imshow("sparse", debug_sparse);

    //    //begin = std::chrono::steady_clock::now();

    //    //cv::Mat flow(keyframe_gray.size(), CV_32FC2);
    //    //calcOpticalFlowFarneback(keyframe_gray, frame_gray, flow,
    //    //        0.5, 3, 15, 3, 5, 1.2, 0);

    //    //end = std::chrono::steady_clock::now();
    //    //printf("Farneback Flow time    = %.4f\n",
    //    //        (float)(std::chrono::duration_cast<std::chrono::milliseconds>(end - begin).count()) / 1000.0f);

    //    //cv::imshow("dense", flow);
    //}
}


/*
#include <iostream>
#include <opencv2/core.hpp>
#include <opencv2/highgui.hpp>
#include <opencv2/imgproc.hpp>
#include <opencv2/videoio.hpp>
#include <opencv2/video.hpp>
using namespace cv;
using namespace std;
int main()
{
    int camera_idx = 2;
    cv::VideoCapture camera(camera_idx);
    if (!camera.isOpened()) {
        printf("Can't open camera %u\n", camera_idx);
        return 0;
    }

    Mat frame1, prvs;
    camera >> frame1;
    cvtColor(frame1, prvs, COLOR_BGR2GRAY);
    while(true){
        Mat frame2, next;
        camera >> frame2;
        if (frame2.empty())
            break;
        cvtColor(frame2, next, COLOR_BGR2GRAY);
        Mat flow(prvs.size(), CV_32FC2);
        calcOpticalFlowFarneback(prvs, next, flow, 0.5, 3, 15, 3, 5, 1.2, 0);
        // visualization
        Mat flow_parts[2];
        split(flow, flow_parts);
        Mat magnitude, angle, magn_norm;
        cartToPolar(flow_parts[0], flow_parts[1], magnitude, angle, true);
        normalize(magnitude, magn_norm, 0.0f, 1.0f, NORM_MINMAX);
        angle *= ((1.f / 360.f) * (180.f / 255.f));
        //build hsv image
        Mat _hsv[3], hsv, hsv8, bgr;
        _hsv[0] = angle;
        _hsv[1] = Mat::ones(angle.size(), CV_32F);
        _hsv[2] = magn_norm;
        merge(_hsv, 3, hsv);
        hsv.convertTo(hsv8, CV_8U, 255.0);
        cvtColor(hsv8, bgr, COLOR_HSV2BGR);
        imshow("frame2", bgr);
        int keyboard = waitKey(30);
        if (keyboard == 'q' || keyboard == 27)
            break;
        prvs = next;
    }
}
*/
