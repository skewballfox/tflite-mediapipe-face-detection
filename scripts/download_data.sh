#!/usr/bin/bash

PROJECT_DIR=$(git rev-parse --show-toplevel)

mkdir $PROJECT_DIR/data
curl -sLO https://storage.googleapis.com/mediapipe-assets/face_detection_full_range_sparse.tflite --output-dir $PROJECT_DIR/data
curl -sL https://avatars.githubusercontent.com/u/29311431?v=4 -o skewballfox.jpg --output-dir $PROJECT_DIR/data
