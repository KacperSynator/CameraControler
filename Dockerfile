FROM kakkosbp/camera-controller_image_base:latest as builder

# Set the working directory inside the container
WORKDIR /app

# Copy the project files into the container
COPY ./camera_controller /app

# Build the application code (e.g., cargo build)
RUN cargo build --release

# Set the entrypoint
CMD cargo run
