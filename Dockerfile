# Specify the base image
FROM balenalib/raspberry-pi:latest

# Update the package lists
RUN apt-get update

# Install dependencies
RUN apt-get -y install libopencv-dev clang libclang-dev libglib2.0-dev gstreamer1.0-rtsp libgstreamer1.0-dev \
 libgstreamer-plugins-base1.0-dev libgstrtspserver-1.0-dev gstreamer1.0-plugins-ugly gstreamer1.0-tools

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set log level to info
ENV RUST_LOG=info

# Set the working directory inside the container
WORKDIR /workspace

# Copy the project files into the container
COPY ./camera_controller /workspace

# Build Rust application
RUN cargo build

# Run your Rust application
CMD cargo run
