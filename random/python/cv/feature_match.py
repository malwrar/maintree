'''
Find features that match between subsequent frames in main system camera.
'''

import cv2
import numpy as np
#import glob

# DETECTOR = cv2.ORB_create()
# DETECTOR = cv2.KAZE_create()
# DETECTOR = cv2.MSER_create()
# DETECTOR = cv2.BRISK_create()
DETECTOR = cv2.AKAZE_create()
#DETECTOR = cv2.AKAZE_create(threshold=0.0001)
#MATCHER = cv2.BFMatcher_create(cv2.NORM_L2, crossCheck=True)
MATCHER = cv2.BFMatcher_create(cv2.NORM_HAMMING, crossCheck=True)

class Capture:
    def __init__(self, img):
        self.img = img

        # Things that will be lazily evaluated later
        self._keypoints = None
        self._descriptors = None

    def keypoints(self):
        self.compute_features()

        return self._keypoints

    def descriptors(self):
        self.compute_features()

        return self._descriptors

    def compute_features(self):
        if self._keypoints is None or self._descriptors is None:
            self._keypoints, self._descriptors = \
                DETECTOR.detectAndCompute(self.img, None)

        return self._keypoints, self._descriptors

def process_pair(cap1, cap2):
    kp1, desc1 = cap1.compute_features()
    kp2, desc2 = cap2.compute_features()


    print(f'len(kp1) = {len(kp1)} len(kp2) = {len(kp2)}')

    matches = MATCHER.match(desc1, desc2)
    matches = sorted(matches, key=lambda x: x.distance)

    match_limit = 100
    out = cv2.drawMatches(cap1.img, kp1, cap2.img, kp2, matches[:match_limit], flags=2, outImg=None)
    cv2.imshow('matches', out)

def main():
    cam_idx = 0
    cam = cv2.VideoCapture(cam_idx)

    print(f'Capturing on cam {cam_idx}.')

    prev = None
    while True:
        key = cv2.waitKey(1)
        if key == ord('q') or key == 27:  # 27 == esc
            break

        _check, frame = cam.read()
        capture = Capture(frame)
        if len(capture.keypoints()) == 0:
            continue

        if prev is not None:
            process_pair(capture, prev)

        prev = capture

    print('Exiting...')
    cam.release()

if __name__ == '__main__':
    main()
    cv2.destroyAllWindows()