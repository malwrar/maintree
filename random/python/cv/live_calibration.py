'''
Calc camera instrinsics given n>2 captures including https://github.com/opencv/opencv/blob/4.x/doc/pattern.png
'''

import sys

import cv2
import numpy as np
 
CLAHE = cv2.createCLAHE(clipLimit = 5)

BOARD_SIZE = (9, 6)
TERMINATION_CRITERIA = (cv2.TERM_CRITERIA_EPS + cv2.TERM_CRITERIA_MAX_ITER, 30, 0.001)

MAX_CAPTURES=200

def main():
    print('Instructions:')
    print('  1.) Print out calibration pattern and place it somewhere it won\'t move.')
    print('  2.) Point camera at the calibration pattern, move it around until we detect the pattern.')
    print('      If detected, a window will pop up with annotations.')
    print(f'  3.) Continue moving the camera around until you capture {MAX_CAPTURES} instances of the pattern.')
    print('  4.) ???')
    print('  5.) Profit!')

    if len(sys.argv) < 2:
        print(f'Usage: {sys.argv[0]} <webcam num>')
        return

    capture_device = cv2.VideoCapture(int(sys.argv[1]))
    if not capture_device:
        print(f'Failed to get capture device {sys.argv[1]}')
        return

    objp = np.zeros((BOARD_SIZE[1]*BOARD_SIZE[0], 3), np.float32)
    objp[:,:2] = np.mgrid[0:BOARD_SIZE[0], 0:BOARD_SIZE[1]].T.reshape(-1,2)   

    imgpoints = []
    objpoints = []
    num_captures = 0

    shape = None

    while True:
        key = cv2.waitKey(1)
        if key == ord('q') or key == 27:  # 27 == esc
            print('Got exit key, breaking early...')
            break
    
        ret, frame = capture_device.read()

        shape = frame.shape
        height, width, _ = shape
    
        # Convert to black and white for easier processing (TODO: can we extract details from color)
        frame_bw = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)
    
        # The declaration of CLAHE
        # clipLimit -> Threshold for contrast limiting
        frame_enhanced = CLAHE.apply(frame_bw)  # + 20
 
        ret, corners = cv2.findChessboardCorners(frame_enhanced, BOARD_SIZE,
                cv2.CALIB_CB_ADAPTIVE_THRESH + cv2.CALIB_CB_FAST_CHECK)
        if not ret:
            #print('FATAL: failed to find chessboard corners in image.')
            continue

        corners = cv2.cornerSubPix(frame_enhanced, corners, (11, 11), (-1, -1),
                TERMINATION_CRITERIA)

        imgpoints.append(corners)
        objpoints.append(objp)

        # Display the resulting frame
        image_debug = cv2.drawChessboardCorners(frame, BOARD_SIZE,
                corners, ret)

        num_captures += 1
        cv2.putText(image_debug, f'{num_captures}', (0, height - 5),
                cv2.FONT_HERSHEY_SIMPLEX, 1.0, (0, 100, 255), 1, cv2.LINE_AA)

        cv2.imshow('frame', image_debug)

        if num_captures >= MAX_CAPTURES:
            print('Reached max captures, breaking...')
            break

    capture_device.release()

    if shape is None:
        print('No captures, can\'t calibrate camera...')
        return

    print('Calculating calibration stuff.')
    ret, mtx, dist, rvecs, tvecs = cv2.calibrateCamera(objpoints, imgpoints,
            shape[:2], None, None)

    print(f'f_x = {float(mtx[0][0])}')
    print(f'f_y = {float(mtx[1][1])}')
    print(f'c_x = {float(mtx[0][2])}')
    print(f'c_y = {float(mtx[1][2])}')
    print(f'k_1 = {dist[0][0]}')
    print(f'k_2 = {dist[0][1]}')
    print(f'p_1 = {dist[0][2]}')
    print(f'p_2 = {dist[0][3]}')
    print(f'k_3 = {dist[0][4]}')

if __name__ == '__main__':
    main()
    cv2.destroyAllWindows()
