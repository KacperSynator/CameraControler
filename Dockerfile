# Specify the base image
FROM rust:latest

# Update the package lists
RUN apt-get update

# Install OpenCV dependencies
RUN apt-get install libopencv-dev clang libclang-dev

# Set the working directory inside the container
WORKDIR /app

# Copy the project files into the container
COPY . .

# Build and run your Rust application
CMD cargo run
