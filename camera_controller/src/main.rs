use log::info;
use opencv::{imgcodecs, Result};
use std::error::Error;

use camera_controller::camera_manager::CameraManager;
use camera_controller::video_streamer::VideoStreamer;

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let mut video_streamer = VideoStreamer::new()?;

    let mut camera_manager = CameraManager::new()?;
    let frame = camera_manager.read_frame()?;

    imgcodecs::imwrite("output.jpg", &frame, &opencv::types::VectorOfi32::new())?;
    info!("Image saved successfully");

    info!("Press q + Enter to exit");
    loop {
        let frame = camera_manager.read_frame()?;
        video_streamer.stream_frame(&frame)?;

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        if input.trim() == "q" {
            break;
        };
    }

    Ok(())
}
