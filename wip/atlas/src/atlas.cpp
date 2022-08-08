#include <stdio.h>

#include <opencv2/opencv.hpp>

#include "atlas.h"

static cv::Ptr<cv::Feature2D> AKAZE = cv::AKAZE::create();
static cv::Ptr<cv::DescriptorMatcher> MATCHER = cv::DescriptorMatcher::create(
        cv::DescriptorMatcher::BRUTEFORCE);

void a() {
    printf("Hello world.\n");
}

Frame::Frame(cv::Mat& image) {
    this->image = preprocessImage(image);

    AKAZE->detect(image, this->features, cv::noArray());
}

Frame::Frame(cv::Mat& image, std::vector<cv::KeyPoint> features) {
    this->image = image;
    this->features = features;
}

Frame Frame::retrack(cv::Mat& image) {
    // Convert keypoints to points.
    std::vector<cv::Point2f> last_points;

    for (size_t i=0; i < this->features.size(); i++) {
        last_points.push_back(this->features[i].pt);
    }

    // Rather than re-detect features and try to match them or something,
    // it's cheaper to just try finding them again using optical flow.
    std::vector<cv::Point2f> retracked_points;
    std::vector<unsigned char> status;
    std::vector<float> error;

    cv::Mat preprocessed_image = preprocessImage(image);

    cv::calcOpticalFlowPyrLK(this->image, preprocessed_image, last_points,
            retracked_points, status, error, cv::Size(9, 9), 4);

    // Copy retracked keypoints to output fec
    std::vector<cv::KeyPoint> retracked_keypoints;

    for (size_t i=0; i < this->features.size(); i++) {
        if (!status[i]) continue;

        cv::KeyPoint feature = features[i];
        feature.pt = retracked_points[i];  // TODO: pretty sure other keypoint metadata from old retracked one should be valid, verify tho.

        retracked_keypoints.push_back(feature);
    }

    return Frame(preprocessed_image, retracked_keypoints);
}

cv::Mat Frame::describeFeatures() {
    // TODO: cache descriptors
    cv::Mat descriptors;

    AKAZE->compute(this->image, features, descriptors);
    assert(features.size() == descriptors.rows);

    return descriptors;
}

void Frame::findMatchingFeatureIndices(Frame& frame) {
    cv::Mat desc1 = this->describeFeatures();
    cv::Mat desc2 = frame.describeFeatures();

    printf("%lu %lu\n", desc1.rows, desc2.rows);

    std::vector<cv::DMatch> matches;
    MATCHER->match(desc1, desc2, matches);

    printf("%lu\n", matches.size());

    cv::Mat debug_img;
    cv::drawMatches(this->image, this->features, frame.image, frame.features, matches, debug_img);

    cv::imshow("matches", debug_img);

    std::vector<int> queryIdxs(matches.size()), trainIdxs(matches.size());

    for (size_t i=0; i < matches.size(); i++) {
        queryIdxs[i] = matches[i].queryIdx;
        trainIdxs[i] = matches[i].trainIdx;
    }

    std::vector<cv::Point2f> points1, points2;

    cv::KeyPoint::convert(this->features, points1, queryIdxs);
    cv::KeyPoint::convert(frame.features, points2, trainIdxs);

    //H1to2 = cv::findHomography(cv::Mat(points1), cv::Mat(points2), 0, ransacReprojThreshold);
    H1to2 = cv::findHomography(cv::Mat(points1), cv::Mat(points2), cv::LMEDS);
    //cv::Mat H1to2 = cv::findHomography(cv::Mat(points1), cv::Mat(points2), cv::RANSAC, 3.0);
    //H1to2 = cv::findHomography(cv::Mat(points1), cv::Mat(points2), cv::RHO, ransacReprojThreshold);

    // Warp image.
    cv::Mat image_warped;
    cv::warpPerspective(this->image, image_warped, H1to2, cv::Size(800, 800), cv::INTER_LINEAR, cv::BORDER_CONSTANT, cv::Scalar());
/*

    // Transform points.
    std::vector<cv::Point2f> points1_transformed;

    cv::perspectiveTransform(cv::Mat(points1), points1_transformed, H1to2);

    // Draw inliers.
    std::vector<char> matchesMask(matches.size(), 0);

    //for (size_t i = 0; i < points1.size(); ++i)
    //    if (cv::norm(points2[i] - points1_transformed.at<cv::Point2f>((int)i, 0)) < 4.0)  // Inlier.
    //        matchesMask[i] = 1;

    //cv::drawMatches(rgb1, keypoints1, rgb2, keypoints2, matches, img_matches, CV_RGB(0, 255, 0), CV_RGB(0, 0, 255), matchesMask, cv::DrawMatchesFlags::DEFAULT);
    //cv::drawMatches(rgb1, keypoints1, rgb2, keypoints2, matches, img_matches, CV_RGB(0, 255, 0), CV_RGB(0, 0, 255), matchesMask, cv::DrawMatchesFlags::DRAW_RICH_KEYPOINTS);

    // Draw outliers.
    //for (size_t i = 0; i < matchesMask.size(); ++i)
    //	matchesMask[i] = !matchesMask[i];

    //cv::drawMatches(rgb1, keypoints1, rgb2, keypoints2, matches, img_matches, CV_RGB(255, 0, 255), CV_RGB(255, 0, 0), matchesMask, cv::DrawMatchesFlags::DRAW_OVER_OUTIMG | cv::DrawMatchesFlags::NOT_DRAW_SINGLE_POINTS);
    //cv::drawMatches(rgb1, keypoints1, rgb2, keypoints2, matches, img_matches);
    */

    //cv::imshow("Feature - Match", img_matches);
    cv::imshow("Feature - Warp", image_warped);

    /*
    # find the keypoints and descriptors with SIFT
    kp1, des1 = sift.detectAndCompute(img1,None)
    kp2, des2 = sift.detectAndCompute(img2,None)

    # FLANN parameters
    FLANN_INDEX_KDTREE = 1
    index_params = dict(algorithm = FLANN_INDEX_KDTREE, trees = 5)
    search_params = dict(checks=50)   # or pass empty dictionary

    flann = cv.FlannBasedMatcher(index_params,search_params)
    matches = flann.knnMatch(des1,des2,k=2)

    # Need to draw only good matches, so create a mask
    matchesMask = [[0,0] for i in range(len(matches))]

    # ratio test as per Lowe's paper
    for i,(m,n) in enumerate(matches):
        if m.distance < 0.7*n.distance:
            matchesMask[i]=[1,0]
    draw_params = dict(matchColor = (0,255,0),
                       singlePointColor = (255,0,0),
                       matchesMask = matchesMask,
                       flags = cv.DrawMatchesFlags_DEFAULT)
    img3 = cv.drawMatchesKnn(img1,kp1,img2,kp2,matches,None,**draw_params)
    */
}

cv::Mat Frame::preprocessImage(cv::Mat& image) {
    cv::Mat image_gray;

    cv::cvtColor(image, image_gray, cv::COLOR_BGR2GRAY);

    return image_gray;
}