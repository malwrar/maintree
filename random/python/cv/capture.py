import cv2

capture_device = cv2.VideoCapture(0)

while True:
    if cv2.waitKey(1) & 0xFF == ord('q'):
        break

    ret, frame = capture_device.read()

    # Convert to black and white for easier processing (TODO: can we extract details from color)
    frame_bw = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)

    # The declaration of CLAHE
    # clipLimit -> Threshold for contrast limiting
    clahe = cv2.createCLAHE(clipLimit = 5)
    frame_enhanced = clahe.apply(frame_bw)  # + 20

    # Display the resulting frame
    cv2.imshow('frame1', frame_enhanced)
    cv2.imshow('frame2', frame_enhanced + 30)

capture_device.release()
cv2.destroyAllWindows()