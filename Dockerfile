# Specify the base image
FROM rust:latest

# Update the package lists
RUN apt-get update

# Install OpenCV dependencies
RUN apt-get -y install libopencv-dev clang libclang-dev libraspberrypi0 libraspberrypi-dev libraspberrypi-bin \
    libglib2.0-dev libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev libgstrtspserver-1.0-dev libcamera-apps

# Set the working directory inside the container
WORKDIR /app

# Copy the project files into the container
COPY ./camera_controller /app

# Build Rust application
RUN cargo build

# Run your Rust application
CMD cargo run
