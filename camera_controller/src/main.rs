use opencv::{imgcodecs, prelude::*, videoio, Result};

fn main() -> Result<()> {
    // Open the camera
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
	if !opened {
		panic!("Unable to open default camera!");
	}

    // Create a Mat to store the captured frame
    let mut frame = Mat::default();

    // Read the frame from the camera
    cam.read(&mut frame).unwrap();

    // Check if the frame is valid
    if frame.size()?.width > 0 {
        // Save the frame as an image file
        imgcodecs::imwrite("output.jpg", &frame, &opencv::types::VectorOfi32::new()).unwrap();

        println!("Image saved successfully");
    } else {
        println!("Failed to capture frame from the camera");
    }

    Ok(())
}
