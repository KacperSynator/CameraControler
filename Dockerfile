# Specify the base image
FROM rust:latest

# Update the package lists
RUN apt-get update

# Install OpenCV dependencies
RUN apt-get -y install libopencv-dev clang libclang-dev

# Set the working directory inside the container
WORKDIR /app

# Copy the project files into the container
COPY ./camera_controller /app

# Build and run your Rust application
CMD cargo run
