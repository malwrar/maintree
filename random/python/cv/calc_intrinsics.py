'''
Calc camera instrinsics given n>2 captures including https://github.com/opencv/opencv/blob/4.x/doc/pattern.png
'''

import cv2
import numpy as np
import glob

BOARD_SIZE = (9, 6)
IMAGE_WIDTH = 1200
TERMINATION_CRITERIA = (cv2.TERM_CRITERIA_EPS + cv2.TERM_CRITERIA_MAX_ITER, 30, 0.001)

def find_corners(image, rows, columns, extent=3):
    for i in range(0, extent):
        for j in range(0, extent):
            print(f'trying offset {i} {j}')
            ret, corners = cv2.findChessboardCorners(image, (rows - i, columns - j), None)
            if ret:
                return ret, corners

    return False, None

def main():
    print('Gathering corners from calib images.')

    objp = np.zeros((BOARD_SIZE[1]*BOARD_SIZE[0], 3), np.float32)
    objp[:,:2] = np.mgrid[0:BOARD_SIZE[0], 0:BOARD_SIZE[1]].T.reshape(-1,2)   

    imgpoints = []
    objpoints = []
    shape = None
    for image in glob.glob('./dataset/iphone/*.jpg'):
        print(f'Processing {image}...')

        image = cv2.imread(image)
        image_gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)

        if not shape:
            shape = image_gray.shape
        elif shape != image_gray.shape:
            print('FATAL: image size doesn\'t match other image sizes.')
            return

        ret, corners = cv2.findChessboardCorners(image_gray, BOARD_SIZE, None)
        if not ret:
            print('FATAL: failed to find chessboard corners in image.')
            return

        corners = cv2.cornerSubPix(
            image_gray, corners,
            (11, 11), (-1, -1),
            TERMINATION_CRITERIA)
        
        imgpoints.append(corners)
        objpoints.append(objp)

        #image = cv2.drawChessboardCorners(image, BOARD_SIZE, corners, ret)
        #image = cv2.resize(image, (int(IMAGE_WIDTH), int(IMAGE_WIDTH * ratio)))
        #cv2.imshow('annotated frame', image)

        #ch = 0xFF & cv2.waitKey(1)
        #if ch == ord('q'):
        #    continue

    ret, mtx, dist, rvecs, tvecs = cv2.calibrateCamera(objpoints, imgpoints, shape, None, None)
    print(f'ret:   {ret}')
    print(f'mtx:   {mtx}')
    print(f'dist:  {dist}')
    print(f'rvecs: {rvecs}')
    print(f'tvecs: {tvecs}')
    
if __name__ == '__main__':
    main()
    cv2.destroyAllWindows()