use log::error;
use opencv::{prelude::*, videoio, Result};
use std::error::Error;
use std::fmt;

const DEFAULT_CAMERA: i32 = 0;

#[derive(Debug)]
struct CameraManagerError(String);

impl fmt::Display for CameraManagerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

impl Error for CameraManagerError {}

pub struct CameraManager {
    camera: videoio::VideoCapture,
}

impl CameraManager {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let camera = videoio::VideoCapture::new(DEFAULT_CAMERA, videoio::CAP_ANY)?;
        let opened = videoio::VideoCapture::is_opened(&camera)?;

        if !opened {
            let error_msg = String::from("Unable to open default camera!");
            error!("{}", error_msg);
            return Err(Box::<CameraManagerError>::new(CameraManagerError(
                error_msg,
            )));
        }

        Ok(Self { camera })
    }

    pub fn read_frame(&mut self) -> Result<Mat, Box<dyn Error>> {
        let mut frame = Mat::default();

        if let Err(err) = self.camera.read(&mut frame) {
            let error_msg = format!("Failed to read frame from the camera: {:?}", err);
            error!("{}", error_msg);
            return Err(Box::<CameraManagerError>::new(CameraManagerError(
                error_msg,
            )));
        }

        if frame.size()?.width <= 0 {
            let error_msg = String::from("Failed to capture frame, width <= 0");
            error!("{}", error_msg);
            return Err(Box::<CameraManagerError>::new(CameraManagerError(
                error_msg,
            )));
        }

        Ok(frame)
    }
}
