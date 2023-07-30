use log::info;
use opencv::{imgcodecs, Result};
use std::error::Error;

use camera_controller::camera_manager::CameraManager;

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let mut camera_manager = CameraManager::new()?;
    let frame = camera_manager.read_frame()?;

    imgcodecs::imwrite("output.jpg", &frame, &opencv::types::VectorOfi32::new())?;
    info!("Image saved successfully");

    Ok(())
}
