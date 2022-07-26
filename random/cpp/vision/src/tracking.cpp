#include <stdio.h>

#include <chrono>
#include <set>
#include <vector>

#include <opencv2/opencv.hpp>
#include <opencv2/imgproc.hpp>
#include <opencv2/line_descriptor.hpp>

#include "tracking.h"

static int IMAGE_WIDTH = 1920;
static int IMAGE_HEIGHT = 1080;

static int CLAHE_CLIP_LIMIT = 40;
static int CLAHE_TILE_SIZE = 50;
static cv::Size CLAHE_TILES(IMAGE_WIDTH / CLAHE_TILE_SIZE,
    IMAGE_HEIGHT / CLAHE_TILE_SIZE);
static cv::Ptr<cv::CLAHE> CLAHE = cv::createCLAHE(CLAHE_CLIP_LIMIT,
    CLAHE_TILES);

static cv::Ptr<cv::Feature2D> FAST = cv::FastFeatureDetector::create(10);

namespace util {
    size_t count_nonzero(std::vector<unsigned char>& mask) {
        size_t count = 0;

        for (size_t i=0; i < mask.size(); i++) {
            if (!mask[i]) { continue; }

            count++;
	}

        return count;
    }

    size_t count_nonzero(cv::Mat& mask) {
        size_t count = 0;

        for (size_t i=0; i < mask.rows; i++) {
            for (size_t j=0; j < mask.cols; j++) {
                if (!mask.at<unsigned char>(i, j)) { continue; }

                count++;
	    }
	}

        return count;
    }

    template <typename T>
    void prune_vector(std::vector<T>& vec, std::vector<unsigned char>& mask) {
        assert(vec.size() == mask.size());

        std::vector<T> old = vec;

        vec.clear();
        vec.reserve(mask.size());

        for (size_t i=0; i < mask.size(); i++) {
            if (!mask[i]) { continue; }

            vec.push_back(old[i]);
        }
    }

    template <typename T>
    void prune_vector(std::vector<T>& vec, cv::Mat& mask) {
        assert(vec.size() == mask.rows);

        std::vector<T> old = vec;

        vec.clear();
        vec.reserve(mask.rows);

        for (size_t i=0; i < mask.rows; i++) {
            if (mask.at<unsigned char>(i, 0) == 0) { continue; }

            vec.push_back(old[i]);
        }
    }

    void print_matrix(cv::Mat mat, const char* label) {
        if (label != NULL) {
            printf("%s = \n", label);
        }

        for (int row=0; row < mat.rows; row++) {
            printf("[ ");

            for (int col=0; col < mat.cols; col++) {
                double entry = mat.at<double>(row, col);
                printf("%.03lf ", entry);
            }

            printf("]\n");
        }
    }

    void form_intrinsic_matrices(
        cv::Mat& K,
        cv::Mat& dist_coef,
        double fx,
        double fy,
        double cx,
        double cy,
        double k1,
        double k2,
        double p1,
        double p2
    ) {
        K = cv::Mat::eye(3, 3, CV_64F);
        K.at<double>(0, 0) = fx;
        K.at<double>(1, 1) = fy;
        K.at<double>(0, 2) = cx;
        K.at<double>(1, 2) = cy;

        dist_coef = cv::Mat(4, 1, CV_64F);
        dist_coef.at<double>(0) = k1;
        dist_coef.at<double>(1) = k2;
        dist_coef.at<double>(2) = p1;
        dist_coef.at<double>(3) = p2;
    }

    bool decomposeE(
        cv::Mat E,
        cv::Mat R1,
        cv::Mat R2,
        cv::Mat t1,
        cv::Mat t2
    ) {
        cv::SVD svd(E, cv::SVD::MODIFY_A);

        double svd_ratio = fabsf(svd.w.at<double>(0) / svd.w.at<double>(1));
        if (svd_ratio > 1.0) svd_ratio = 1.0 / svd_ratio;

        if (svd_ratio < 0.7) {
            printf("Singlar values of E are too far apart!\n");
	    return false;
	}

        cv::Matx33d W( 0, -1,  0,
                       1,  0,  0,
                       0,  0,  1);

        cv::Matx33d Wt(0,  1,  0,
                      -1,  0,  0,
                       0,  0,  1);

        R1 =  svd.u * cv::Mat(W) * svd.vt;
        R2 =  svd.u * cv::Mat(W) * svd.vt;
        t1 =  svd.u.col(2);
        t2 = -svd.u.col(2);

        return true;
    }

    /**
     *
     */
    bool triangulate(
        const std::vector<cv::Point2f>& p1,
        const std::vector<cv::Point2f>& p2,
        std::vector<cv::Point3d>& p3,
        std::vector<unsigned char>& status,
        cv::Mat K,
        cv::Mat E
    ) {
        cv::Mat_<double> R1(3, 3);
        cv::Mat_<double> R2(3, 3);
        cv::Mat_<double> t1(1, 3);
        cv::Mat_<double> t2(1, 3);

        if (!decomposeE(E, R1, R2, t1, t2)) {
            printf("Failed to decompose E.");
            return false;
        }

        if (cv::determinant(R1) + 1.0 < 1e-09) {
            if (decomposeE(-E, R1, R2, t1, t2)) {
                printf("Failed to decompose -E.");
                return false;
            }
        }

        if (fabsf(cv::determinant(R1)) - 1.0 > 1e-07) {
            printf("R1 is not a rotation matrix.");
            return false;
        }

        return triangulate(p1, p2, p3, status, K, R1, R2, t1, t2);
    }

    bool triangulate(
        const std::vector<cv::Point2f>& p1,
        const std::vector<cv::Point2f>& p2,
        std::vector<cv::Point3d>& p3,
        std::vector<unsigned char>& status,
        cv::Mat_<double> K,
        cv::Mat_<double> R1,
        cv::Mat_<double> R2,
        cv::Mat_<double> t1,
        cv::Mat_<double> t2
    ) {
        if (triangulate(p1, p2, p3, status, K, R1, t1)) return true;
        if (triangulate(p1, p2, p3, status, K, R2, t1)) return true;
        if (triangulate(p1, p2, p3, status, K, R1, t2)) return true;
        if (triangulate(p1, p2, p3, status, K, R2, t2)) return true;

        printf("Failed to triangulate for all possible rotation and translation matrices.\n");
        return false;
    }

    bool triangulate(
        const std::vector<cv::Point2f>& p1,
        const std::vector<cv::Point2f>& p2,
        std::vector<cv::Point3d>& p3,
        std::vector<unsigned char>& status,
        cv::Mat_<double> K,
        cv::Mat_<double> R,
        cv::Mat_<double> t
    ) {
        cv::Mat P = cv::Mat::eye(3, 4, CV_64FC1);
        cv::Mat_<double> P1 = (cv::Mat_<double>(3,4) <<
            R(0,0), R(0,1), R(0,2), t(0),
            R(1,0), R(1,1), R(1,2), t(1),
            R(2,0), R(2,1), R(2,2), t(2));

        cv::Mat p3_h(4, p1.size(), CV_32FC1);
        cv::triangulatePoints(P, P1, p1, p2, p3_h);

        cv::Mat p3_e;  // euclidian
        cv::convertPointsFromHomogeneous(cv::Mat(p3_h.t()).reshape(4, 1), p3_e);

        status.resize(p3_e.rows, 0);
        p3.clear();

        for (int i=0; i < p3_e.rows; i++) {
            auto p = p3_e.at<cv::Point3d>(i);

            // Is point in front?
            if (p.z > 0) {
                status[i] = 1;
                p3.push_back(p);
            }
        }

        printf("p3   %lu\n", p3.size());
        printf("p3_e %lu %lu\n", p3_e.rows, p3_e.cols);

        double percentage = (double)p3.size() / (double)p3_e.rows;
        printf("%.01f / %.01f = %.02f, < * 100 = %d\n", (double)p3.size(), (double)p3_e.rows, percentage, percentage * 100.0);
        if (percentage < 0.75) {
            printf("Failed to track > 75%% of points, only managed %d%%!\n",
                    (size_t)(100.0 * percentage));
            return false;
        }

        /*
        // Calculate reprojection
        cv::Mat_<double> R = P(cv::Rect(0, 0, 3, 3));
        cv::Vec3d rvec(0,0,0); //Rodrigues(R ,rvec);
        cv::Vec3d tvec(0,0,0); // = P.col(3);

        std::vector<cv::Point2f> reprojected_pt_set1;
        cv::projectPoints(p3_e, rvec, tvec, K, Mat(),reprojected_pt_set1);
//    cout << Mat(reprojected_pt_set1).rowRange(0,10) << endl;
        std::vector<cv::Point2f> bootstrapPts_v = Points<float>(bootstrap_kp);
        Mat bootstrapPts = Mat(bootstrapPts_v);
//    cout << bootstrapPts.rowRange(0,10) << endl;

        double reprojErr = cv::norm(Mat(reprojected_pt_set1),bootstrapPts,NORM_L2)/(double)bootstrapPts_v.size();
        cout << "reprojection Error " << reprojErr;
        if(reprojErr < 5) {
            vector<uchar> status(bootstrapPts_v.size(),0);
            for (int i = 0;  i < bootstrapPts_v.size(); ++ i) {
                status[i] = (norm(bootstrapPts_v[i]-reprojected_pt_set1[i]) < 20.0);
            }

            trackedFeatures3D.clear();
            trackedFeatures3D.resize(pt_3d.rows);
            pt_3d.copyTo(Mat(trackedFeatures3D));

            keepVectorsByStatus(trackedFeatures,trackedFeatures3D,status);
            cout << "keeping " << trackedFeatures.size() << " nicely reprojected points";
            bootstrapping = false;
            return true;
        }
        return false;
        */

        return true;
    }
}

/**
 *
 */
enum TrackerState {
    Bootstrapping,
    Localizing,
    Tracking,
};

/**
 * Given a continuous set of sequential, close captures from a single monocular
 * camera, this class will attempt to track the 6 DoF pose of the camera in
 * real time while simultaneously mapping the surrounding environment.
 *
 * Note that this class does not perform any offline refinement, and will
 * likely drift over large distances.
 */
class Tracker {
public:
    Tracker() : state(TrackerState::Bootstrapping) { }
 
    /**
     * Submit next image from camera for tracking.
     */
    bool submit(
        const cv::Mat& image,
        cv::Mat K,
        cv::Mat dist_coef
    ) {
        printf("features: %lu\n", last_points_2d.size());
        cv::Mat processed_image = enhanceImage(image);

        switch (state) {
        case TrackerState::Bootstrapping:
            return bootstrap(processed_image, K, dist_coef);
        case TrackerState::Localizing:
            return localize(processed_image, K, dist_coef);
        case TrackerState::Tracking:
            return track(processed_image, K, dist_coef);
        default:
            return false;
	}
    }

    bool debug(
        cv::Mat& out_image,
	bool use_unprocessed_image=true
    ) {
        // Draw 2d keypoints.
        for (int i=0; i < last_points_2d.size(); i++) {
            auto point = last_points_2d[i];

            int x = point.x + 0.5;
            int y = point.y + 0.5;

            cv::circle(out_image, cv::Point(x, y), 1.0, cv::Scalar(255, 0, 0), -1);
        }

        return true;
    }

private:
    TrackerState state;

    cv::Mat last_image;
    std::vector<cv::Point2f> last_points_2d;
    std::vector<cv::Point3d> points_3d;

    cv::Mat t_aux, r_aux;

    /**
     *
     */
    bool bootstrap(
        const cv::Mat& image,
        cv::Mat K,
        cv::Mat dist_coef
    ) {
        reset();

        last_image = image;

        std::vector<cv::KeyPoint> features;
        FAST->detect(last_image, features);

        last_points_2d.reserve(features.size());
        for (auto feature : features) {
            last_points_2d.push_back(feature.pt);
        }

        state = TrackerState::Localizing;

        return false;
    }

    /**
     *
     */
    bool localize(
        const cv::Mat& image,
        cv::Mat K,
        cv::Mat dist_coef
    ) {
        if (last_points_2d.size() < 10) {
            state = TrackerState::Bootstrapping;
            return false;
        }

        // Forward optical flow tracking
        std::vector<unsigned char> status(last_points_2d.size());
        std::vector<float> error(last_points_2d.size());

        std::vector<cv::Point2f> retracked_points(last_points_2d.size());

        auto begin = std::chrono::steady_clock::now();

        cv::calcOpticalFlowPyrLK(last_image, image, last_points_2d,
                retracked_points, status, error, cv::Size(9, 9), 5);

        auto end = std::chrono::steady_clock::now();
	//printf("forward tracking    %.3f\n", (float)std::chrono::duration_cast<std::chrono::microseconds>(end - begin).count() / 1000000.0);

        util::prune_vector(last_points_2d, status);
        util::prune_vector(retracked_points, status);

        //printf("Points eliminated in forward optical flow:    %lu\n",
        //        status.size() - util::count_nonzero(status));

	/*
        // Backwards optical flow retracking.
        status.clear();
        error.clear();

        status.reserve(retracked_points.size());
        error.reserve(retracked_points.size());

        std::vector<cv::Point2f> backtracked_points(retracked_points.size());

        begin = std::chrono::steady_clock::now();

        cv::calcOpticalFlowPyrLK(image, last_image, retracked_points,
                backtracked_points, status, error, cv::Size(9, 9), 1);

        end = std::chrono::steady_clock::now();
	//printf("backward tracking   %.3f\n", (float)std::chrono::duration_cast<std::chrono::microseconds>(end - begin).count() / 1000000.0);

        util::prune_vector(last_points_2d, status);
        util::prune_vector(retracked_points, status);
	*/

        //printf("Points eliminated by backward optical flow:   %lu\n",
        //        status.size() - util::count_nonzero(status));

        if (last_points_2d.size() < 4) {
            state = TrackerState::Bootstrapping;
            return false;
        }

        begin = std::chrono::steady_clock::now();

        cv::Mat H, mask;
        H = cv::findHomography(last_points_2d, retracked_points, cv::RANSAC, 3.0, mask);

        cv::Mat image_warped;
        cv::warpPerspective(last_image, image_warped, H, cv::Size(800, 800), cv::INTER_LINEAR, cv::BORDER_CONSTANT, cv::Scalar());

        cv::imshow("warped", image_warped);

        end = std::chrono::steady_clock::now();
        //printf("homography tracking %.3f\n", (float)std::chrono::duration_cast<std::chrono::microseconds>(end - begin).count() / 1000000.0);

        util::prune_vector(last_points_2d, mask);
        util::prune_vector(retracked_points, mask);

        if (last_points_2d.size() < 10) {
            state = TrackerState::Bootstrapping;
            return false;
        }

        //printf("Points eliminated by homography verification: %lu\n",
        //        mask.rows - util::count_nonzero(mask));

        // asdf
        cv::Mat rigidT = cv::estimateAffinePartial2D(last_points_2d, retracked_points);
        printf("cv::norm(rigidT.col(2)) = %lf\n", cv::norm(rigidT.col(2)));
        if (cv::norm(rigidT.col(2)) > 100) {
            printf("Attempting triangulation\n");

            // Attempt triangulation
            status.clear();
            status.reserve(last_points_2d.size());

            begin = std::chrono::steady_clock::now();

            cv::Mat F = cv::findFundamentalMat(last_points_2d, retracked_points, cv::FM_RANSAC, 3, 0.99, status);
            end = std::chrono::steady_clock::now();

            printf("fundamental matrix  %.3f\n", (float)std::chrono::duration_cast<std::chrono::microseconds>(end - begin).count() / 1000000.0);
            if (!F.empty()) {
                util::prune_vector(last_points_2d, status);
                util::prune_vector(retracked_points, status);

                printf("Points eliminated by fundamental matrix:      %lu\n",
                        status.size() - util::count_nonzero(status));

                cv::Mat E = K.t() * F * K;
                if (fabsf(cv::determinant(E)) <= 1e-07) {
                    printf("det(E) = %f\n", cv::determinant(E));

                    std::vector<cv::Point2f> p1, p2;

                    cv::undistortPoints(last_points_2d,   p1, K, cv::Mat());
                    cv::undistortPoints(retracked_points, p2, K, cv::Mat());

                    if (util::triangulate(p1, p2, points_3d, status, K, E)) {
                        printf("aaa %lu %lu\n", last_points_2d.size(), status.size());
                        util::prune_vector(last_points_2d, status);
                        util::prune_vector(retracked_points, status);

			for (size_t i=0; i < points_3d.size(); i++) {
                            auto point = points_3d[i];

                            printf("%03u: [%lf, %lf, %lf]\n", i, point.x, point.y, point.z);
			}

                        state = TrackerState::Tracking;
                        return true;
                    }
                }
            }
        }

        // Save last 
        last_points_2d = retracked_points;
        last_image = image;

        return false;
    }

    /**
     *
     */
    bool track(
        const cv::Mat& image,
        cv::Mat K,
        cv::Mat dist_coef
    ) {
        //printf("Tracking!\n");

        // Forward optical flow tracking
        std::vector<unsigned char> status(last_points_2d.size());
        std::vector<float> error(last_points_2d.size());

        std::vector<cv::Point2f> retracked_points(last_points_2d.size());

        cv::calcOpticalFlowPyrLK(last_image, image, last_points_2d,
                retracked_points, status, error, cv::Size(9, 9), 5);

        util::prune_vector(points_3d, status);
        util::prune_vector(last_points_2d, status);
        util::prune_vector(retracked_points, status);

	// Minimum needed for PnP (P3P solution).
	if (points_3d.size() < 3) {
            state = TrackerState::Bootstrapping;
            return false;
	}

        cv::solvePnP(points_3d, retracked_points, K, dist_coef, r_aux, t_aux, !r_aux.empty());

	util::print_matrix(r_aux, "raux");
	util::print_matrix(t_aux, "taux");

	cv::Mat r_vec, t_vec;

	r_aux.convertTo(r_vec, CV_32F);
	t_aux.convertTo(t_vec, CV_64F);

	cv::Mat rot(3, 3, CV_32FC1);
	cv::Rodrigues(r_vec, rot);

	cv::Mat_<double> para = cv::Mat_<double>::eye(4, 4);
	rot.convertTo(para(cv::Rect(0, 0, 3, 3)), CV_64F);
	t_vec.copyTo(para(cv::Rect(3, 0, 1, 3)));

	//para = cvToGl * para;

	//Mat(para.t()).copyTo(modelview);

        last_points_2d = retracked_points;
        last_image = image;

        return true;
    }

    /**
     *
     */
    void reset() {
        state = TrackerState::Bootstrapping;
        last_points_2d.clear();
    }

    /**
     *
     */
    cv::Mat enhanceImage(const cv::Mat& image) {
        // Do some basic preprocesing on the original image.
        cv::Mat image_gray, image_processed;

        cv::cvtColor(image, image_gray, cv::COLOR_BGR2GRAY);
        CLAHE->apply(image_gray, image_processed);

	return image_processed;
    }

    /**
     *
     */
    void relocateFeatures() {

    }

    /**
     *
     */
    void filterNonHomographies(std::vector<cv::Point2f>& p1, std::vector<cv::Point2f>& p2) {

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

    Tracker tracker;

    while (true) {
        // Exit if esc or q key is pressed
        int key = cv::waitKey(1);
        if (key == 'q' || key == 27 /* ESC */) {
            break;
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

        // Process next frame
        cv::Mat frame;
        camera >> frame;

        tracker.submit(frame, K, dist_coef);

        if (tracker.debug(frame))
            cv::imshow("tracking", frame);
    }
}
