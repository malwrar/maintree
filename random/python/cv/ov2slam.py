'''
Attempt at implementing ov2slam: https://arxiv.org/pdf/2102.04060.pdf
'''

import cv2

class App:
    def __init__(self, device):
        self.track_len = 10
        self.detect_interval = 5
        self.tracks = []
        self.cam = device
        self.frame_idx = 0
        self.clahe = cv2.createCLAHE(clipLimit = 5)
        self.fast = cv2.FastFeatureDetector_create()
        self.orb = cv2.ORB_create(nfeatures=2000, nlevels=8)
        self.brief = cv2.xfeatures2d.BriefDescriptorExtractor_create()

    def run(self):
        while True:
            ch = 0xFF & cv2.waitKey(1)
            if ch == ord('q'):
                break

            ret, frame = self.cam.read()

            # Preprocess camera image
            preprocessed_frame = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)
            preprocessed_frame = self.clahe.apply(preprocessed_frame)
            cv2.imshow('preprocessed_frame', preprocessed_frame)

            # Compute ORB-ish features, not sure why ov2slam doesn't directly mention it in the paper
            fast_keypoints = self.fast.detect(preprocessed_frame)
            brief_keypoints, brief_descriptors = self.brief.compute(preprocessed_frame, fast_keypoints)

            # Compute actual ORB features
            orb_keypoints = self.orb.detect(preprocessed_frame, None)
            orb_keypoints, orb_descriptors = self.orb.compute(preprocessed_frame, orb_keypoints)

            # Annotate frame
            annotated_frame = frame  # So I can comment out lines
            annotated_frame = cv2.drawKeypoints(annotated_frame, fast_keypoints, None, color=(255, 0, 0))
            annotated_frame = cv2.drawKeypoints(annotated_frame, brief_keypoints, None, color=(0, 255, 0))
            annotated_frame = cv2.drawKeypoints(annotated_frame, orb_keypoints, None, color=(255, 0, 255))
            annotated_frame = cv2.putText(annotated_frame, f"{brief_descriptors.shape}", (0, 25),
                cv2.FONT_HERSHEY_SIMPLEX, 1, (255, 255, 0), 1, cv2.LINE_AA)
            annotated_frame = cv2.putText(annotated_frame, f"{orb_descriptors.shape}", (0, 75),
                cv2.FONT_HERSHEY_SIMPLEX, 1, (255, 255, 0), 1, cv2.LINE_AA)
            cv2.imshow('annotated_frame', annotated_frame)

def main():
    capture_device = cv2.VideoCapture(0)
    App(capture_device).run()
    cv2.destroyAllWindows()

if __name__ == '__main__':
    main()
