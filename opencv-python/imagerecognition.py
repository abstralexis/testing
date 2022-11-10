"""
Code from RealPython, I am gonna tinker with this a bit
and add comments to this.

Uses one of the pre-trained HAAR Cascades.
"""

import cv2
import sys

cascPath = sys.argv[1] # Gets the argument for the cascade path from cmdline
faceCascade = cv2.CascadeClassifier(cascPath) # Initialise HAAR cascade

video_capture = cv2.VideoCapture(0) # Capture video from video device

while True:
    # Capture frame-by-frame
    ret, frame = video_capture.read()   # Read video frame

    gray = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)  # Get greyscale version

    faces = faceCascade.detectMultiScale(
        gray, # Use greyscale version to detect things
        scaleFactor=1.1, # Change scale factor of image
        minNeighbors=5, # Minium face detections nearby to be recognised
        minSize=(30, 30), # Minimum face detection size px
        flags=cv2.CASCADE_SCALE_IMAGE # Get the xml flags for image recog
        # I had to fix the flags thing as cv2.cv is deprecated.
    )

    # Draw a rectangle around the faces
    for (x, y, w, h) in faces:
        cv2.rectangle(frame, (x, y), (x+w, y+h), (0, 255, 0), 2)

    # Display the resulting frame
    cv2.imshow('Video', frame)

    # What does this memory address looking thing do?
    if cv2.waitKey(1) & 0xFF == ord('q'):
        break   # Quit if q pressed

# When everything is done, release the capture
video_capture.release()
cv2.destroyAllWindows()
