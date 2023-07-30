use opencv::{imgcodecs, prelude::*, videoio, Result};
use std::error::Error;
use log::{error, info};

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    // Open the camera
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
	if !opened {
        error!("Unable to open default camera!");
	}

    // Create a Mat to store the captured frame
    let mut frame = Mat::default();

    // Read the frame from the camera
    cam.read(&mut frame).unwrap();

    // Check if the frame is valid
    if frame.size()?.width > 0 {
        // Save the frame as an image file
        imgcodecs::imwrite("output.jpg", &frame, &opencv::types::VectorOfi32::new()).unwrap();
        info!("Image saved successfully");
    } else {
        error!("Failed to capture frame from the camera");
    }

    Ok(())
}
