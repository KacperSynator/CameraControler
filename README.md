# CameraController

An app for handling the camera on raspberry pi using Rust and OpenCV.
Main functions of the app are:
* Taking pictures
* Streaming videos
* Using OpenCV for detecting objects

### Docker


#### Setup docker and pull image
```bash
# install docker
curl -sSL https://get.docker.com | sh

# add username to docker group
sudo usermod -aG docker <username>
# logout and login to apply changes

# pull docker image
docker pull kakkosbp/camer-trap_image:latest
```

#### Run docker image
```bash
# to run the app use, make sure that camera is connected to /dev/video0
docker run --device=/dev/video0 kakkosbp/camera-controller_image:latest

```

#### Run docker image for development
```bash
# make sure that you cloned the repo first
git clone https://github.com/KacperSynator/CameraController.git
cd CameraController

# for development purposes use interactive session, make sure that camera is connected to /dev/video0
docker run -it --volume "$(pwd)/camera_controller:/workspace" --device=/dev/video0 \
 kakkosbp/camera-controller_image:latest /bin/bash

# from inside the image you can build and run the app
cd workspace && cargo run
```