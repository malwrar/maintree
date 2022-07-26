#include <cmath>
#include <algorithm>

#include "triangulation.h"

namespace math {

std::vector<cv::Point2f> normalize(
    std::vector<cv::Point2f> points,
    cv::Mat& T
) {
    float mean_x = 0.0f;
    float mean_y = 0.0f;

    for (auto point : points) {
        mean_x += point.x;
        mean_y += point.y;
    }

    mean_x /= points.size();
    mean_y /= points.size();

    std::vector<cv::Point2f> normalized(points.size());

    float mean_dev_x = 0.0f;
    float mean_dev_y = 0.0f;

    for (size_t i=0; i < points.size(); i++) {
        normalized[i].x = points[i].x - mean_x;
        normalized[i].y = points[i].y - mean_y;

        mean_dev_x += fabs(normalized[i].x);
        mean_dev_y += fabs(normalized[i].y);
    }

    mean_dev_x /= points.size();
    mean_dev_y /= points.size();

    float s_x = 1.0f / mean_dev_x;
    float s_y = 1.0f / mean_dev_y;

    for (size_t i=0; i < points.size(); i++) {
        normalized[i].x *= s_x;
        normalized[i].y *= s_y;
    }

    T = cv::Mat::eye(3, 3, CV_32F);
    T.at<float>(0, 0) = s_x;
    T.at<float>(1, 1) = s_y;
    T.at<float>(0, 2) = -mean_x * s_x;
    T.at<float>(1, 2) = -mean_y * s_y;

    return normalized;
}

cv::Mat triangulate_pair(
    cv::Point2f p1,
    cv::Point2f p2,
    cv::Mat P1,
    cv::Mat P2
) {
    cv::Mat A(4, 4, CV_32F);

    A.row(0) = p1.x * P1.row(2) - P1.row(0);
    A.row(1) = p1.y * P1.row(2) - P1.row(1);
    A.row(2) = p2.x * P2.row(2) - P2.row(0);
    A.row(3) = p2.y * P2.row(2) - P2.row(1);

    cv::Mat u, w, vt;
    cv::SVD::compute(A, w, u, vt, cv::SVD::MODIFY_A|cv::SVD::FULL_UV);

    cv::Mat p3d = vt.row(3).t();
    return p3d.rowRange(0, 3) / p3d.at<float>(3);
}

int checkRT(
    const cv::Mat& R,
    const cv::Mat& T,
    const std::vector<cv::Point2f>& p1,
    const std::vector<cv::Point2f>& p2,
    const cv::Mat& K,
    float th2,
    std::vector<cv::Point3f>& p3d,
    float& parallax
) {
    const float fx = K.at<float>(0, 0);
    const float fy = K.at<float>(1, 1);
    const float cx = K.at<float>(0, 2);
    const float cy = K.at<float>(1, 2);

    cv::Mat P1(3, 4, CV_32F, cv::Scalar(0));
    K.copyTo(P1.rowRange(0, 3).colRange(0, 3));

    cv::Mat P2(3, 4, CV_32F);
    R.copyTo(P2.rowRange(0, 3).colRange(0, 3));
    T.copyTo(P2.rowRange(0, 3).col(3));
    P2 = K * P2;

    cv::Mat O1 = cv::Mat::zeros(3, 1, CV_32F);
    cv::Mat O2 = -R.t() * T;

    int num_good = 0;
    std::vector<float> cos_parallaxes;
    
    for (size_t i=0; i < p1.size(); i++) {
        cv::Mat p3dC1 = triangulate_pair(p1[i], p2[i], P1, P2);
        if (!std::isfinite(p3dC1.at<float>(0))
                || !std::isfinite(p3dC1.at<float>(0))
                || !std::isfinite(p3dC1.at<float>(0)))
            continue;

        // Check parallax
        cv::Mat normal1 = p3dC1 - O1;
        float dist1 = cv::norm(normal1);

        cv::Mat normal2 = p3dC1 - O2;
        float dist2 = cv::norm(normal2);

        float cos_parallax = normal1.dot(normal2) / (dist1 * dist2);
        if (p3dC1.at<float>(2) <= 0 && cos_parallax < 0.99998)
            continue;

        cv::Mat p3dC2 = R * p3dC1 + T;
        if (p3dC2.at<float>(2) <= 0 && cos_parallax < 0.99998)
            continue;

        // Check reprojection error in first image.
        float invZ1 = 1.0 / p3dC1.at<float>(2);
        float im1x = fx * p3dC1.at<float>(0) * invZ1 + cx;
        float im1y = fy * p3dC1.at<float>(1) * invZ1 + cy;

        float square_err_1 = (im1x - p1[i].x) * (im1x - p1[i].x)
            + (im1y - p1[i].y) * (im1y - p1[i].y);

        if (square_err_1 > th2)
            continue;

        // Check reprojection error in first image.
        float invZ2 = 1.0 / p3dC2.at<float>(2);
        float im2x = fx * p3dC2.at<float>(0) * invZ2 + cx;
        float im2y = fy * p3dC2.at<float>(1) * invZ2 + cy;

        float square_err_2 = (im2x - p2[i].x) * (im2x - p2[i].x)
            + (im2y - p2[i].y) * (im2y - p2[i].y);

        if (square_err_2 > th2)
            continue;

        cos_parallaxes.push_back(cos_parallax);
        p3d.push_back(cv::Point3f(p3dC1.at<float>(0), p3dC1.at<float>(1), p3dC1.at<float>(2)));
        num_good++;
    }

    if (num_good > 0) {
        std::sort(cos_parallaxes.begin(), cos_parallaxes.end());
        size_t idx = std::min(50, int(cos_parallaxes.size() - 1));
        parallax = std::acos(cos_parallaxes[idx]) * (180.0f/CV_PI);
    } else {
        parallax = 0;
    }

    return num_good;
}

void decomposeE(
    const cv::Mat& E,
    cv::Mat& R1,
    cv::Mat& R2,
    cv::Mat& T1,
    cv::Mat& T2
) {
    cv::Mat u, w, vt;
    cv::SVD::compute(E, w, u, vt);

    u.col(2).copyTo(T1);
    T1 = T1 / cv::norm(T1);
    T2 = -T1;

    cv::Mat W(3, 3, CV_32F, cv::Scalar(0));
    W.at<float>(0, 1) = -1;
    W.at<float>(1, 0) = 1;
    W.at<float>(2, 2) = 1;

    R1 = u * W * vt;
    if (cv::determinant(R1) < 0)
        R1 = -R1;

    R2 = u * W.t() * vt;
    if (cv::determinant(R2) < 0)
        R2 = -R2;
}

bool triangulateF(
    std::vector<cv::Point2f> p1,
    std::vector<cv::Point2f> p2,
    cv::Mat F21,
    cv::Mat K,
    cv::Mat& R21,
    cv::Mat& T21,
    std::vector<cv::Point3f>& p3d
) {
    assert(p1.size() == p2.size());

    cv::Mat E21 = K.t() * F21 * K;

    cv::Mat R1, R2, T1, T2;
    decomposeE(E21, R1, R2, T1, T2);

    std::vector<cv::Point3f> p3d_1, p3d_2, p3d_3, p3d_4;
    float parallax1, parallax2, parallax3, parallax4;

    int good1 = checkRT(R1, T1, p1, p2, K, 4.0, p3d_1, parallax1);
    int good2 = checkRT(R2, T1, p1, p2, K, 4.0, p3d_2, parallax2);
    int good3 = checkRT(R1, T2, p1, p2, K, 4.0, p3d_3, parallax3);
    int good4 = checkRT(R2, T2, p1, p2, K, 4.0, p3d_4, parallax4);

    int max_good = std::max(good1, std::max(good2, std::max(good3, good4)));

    int num_similar = 0;
    if (good1 > 0.7 * max_good)
        num_similar++;
    if (good2 > 0.7 * max_good)
        num_similar++;
    if (good3 > 0.7 * max_good)
        num_similar++;
    if (good4 > 0.7 * max_good)
        num_similar++;

    if (num_similar > 1) {
        return false;
    }

    if (max_good == good1) {
        p3d = p3d_1;

        R1.copyTo(R21);
        T1.copyTo(T21);

        return true;
    } else if (max_good == good2) {
        p3d = p3d_2;

        R2.copyTo(R21);
        T1.copyTo(T21);

        return true;
    } else if (max_good == good3) {
        p3d = p3d_3;

        R1.copyTo(R21);
        T2.copyTo(T21);

        return true;
    } else if (max_good == good4) {
        p3d = p3d_4;

        R2.copyTo(R21);
        T2.copyTo(T21);

        return true;
    }

    return false;
}

}