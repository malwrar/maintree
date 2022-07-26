#include "util.h"

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
    cv::Mat E
) {
    cv::Mat_<double> R1(3, 3);
    cv::Mat_<double> R2(3, 3);
    cv::Mat_<double> t1(1, 3);
    cv::Mat_<double> t2(1, 3);

    if (decomposeE(E, R1, R2, t1, t2)) {
        if (cv::determinant(R1) + 1.0 >= 1e-09) {
            return triangulate(p1, p2, p3, R1, R2, t1, t2);
        }
    }

    if (decomposeE(-E, R1, R2, t1, t2)) {
        if (cv::determinant(R1) - 1.0 <= 1e-07) {
            return triangulate(p1, p2, p3, R1, R2, t1, t2);
        }
    }

    printf("Failed to decompose both E and -E!\n");
    return false;
}

bool triangulate(
    const std::vector<cv::Point2f>& p1,
    const std::vector<cv::Point2f>& p2,
    std::vector<cv::Point3d>& p3,
    cv::Mat_<double> R1,
    cv::Mat_<double> R2,
    cv::Mat_<double> t1,
    cv::Mat_<double> t2
) {
    if (triangulate(p1, p2, p3, R1, t1)) return true;
    if (triangulate(p1, p2, p3, R2, t1)) return true;
    if (triangulate(p1, p2, p3, R1, t2)) return true;
    if (triangulate(p1, p2, p3, R2, t2)) return true;

    printf("Failed to triangulate for all possible rotation and translation matrices.\n");
    return false;
}

bool triangulate(
    const std::vector<cv::Point2f>& p1,
    const std::vector<cv::Point2f>& p2,
    std::vector<cv::Point3d>& p3,
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

    p3.clear();

    for (int i=0; i < p3_e.rows; i++) {
        auto p = p3_e.at<cv::Point3d>(i);

        // Is point in front?
        if (p.z > 0) {
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

    // TODO: validate by calculating reprojection.

    return true;
}


}