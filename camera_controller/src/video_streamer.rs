use gst::prelude::Cast;
use gst::prelude::ElementExt;
use gst::prelude::GstBinExt;
use gstreamer as gst;
use gstreamer_app as gst_app;
use log::{error, info};
use opencv::core::Mat;
use opencv::prelude::MatTraitConst;
use opencv::prelude::MatTraitConstManual;
use std::error::Error;
use std::fmt;

const LAUNCH_OPTIONS: &str = "appsrc name=source format=time ! videoconvert ! video/x-raw,format=I420 ! x264enc ! rtspclientsink name=sink location=rtsp://localhost:127/test";

#[derive(Debug)]
struct VideoStreamerError(String);

impl fmt::Display for VideoStreamerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

impl Error for VideoStreamerError {}

pub struct VideoStreamer {
    pipeline: gst::Pipeline,
    is_streaming: bool,
}

impl VideoStreamer {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        if let Err(err) = gst::init() {
            let error_msg = format!("Failed to init gstreamer: {:?}", err);
            error!("{}", error_msg);
            return Err(Box::<VideoStreamerError>::new(VideoStreamerError(
                error_msg,
            )));
        }

        let pipeline = VideoStreamer::create_pipeline()?;

        Ok(Self {
            pipeline,
            is_streaming: false,
        })
    }

    pub fn stream_frame(&mut self, frame: &Mat) -> Result<(), Box<dyn Error>> {
        if !self.is_streaming {
            self.is_streaming = true;
            self.pipeline.set_state(gst::State::Playing)?;
            info!("Starting stream...");
        }

        let appsrc = VideoStreamer::create_appsrc(&self.pipeline)?;

        let data = frame.data();
        let size = frame.total() * frame.elem_size()?;

        let mut buffer = gst::Buffer::with_size(size)?;
        {
            let buffer = buffer.get_mut().unwrap();
            let _ = buffer.copy_from_slice(0, unsafe { std::slice::from_raw_parts(data, size) });

            appsrc.push_buffer(buffer.to_owned())?;
        }

        Ok(())
    }

    fn create_pipeline() -> Result<gst::Pipeline, Box<dyn Error>> {
        let pipeline = gst::parse_launch(LAUNCH_OPTIONS)?;
        let pipeline = pipeline.dynamic_cast::<gst::Pipeline>().map_err(|err| {
            let error_msg = format!("Failed to cast Element to pipeline: {:?}", err);
            error!("{}", error_msg);
            Box::<VideoStreamerError>::new(VideoStreamerError(error_msg))
        })?;

        Ok(pipeline)
    }

    fn create_appsrc(pipeline: &gst::Pipeline) -> Result<gst_app::AppSrc, Box<dyn Error>> {
        let appsrc = pipeline
            .by_name("source")
            .ok_or_else(|| {
                let error_msg = "Failed to get source from pipeline";
                error!("{}", error_msg);
                Box::<VideoStreamerError>::new(VideoStreamerError(error_msg.into()))
            })?
            .dynamic_cast::<gst_app::AppSrc>()
            .map_err(|err| {
                let error_msg = format!("Failed to cast Pipeline to AppSrc: {:?}", err);
                error!("{}", error_msg);
                Box::<VideoStreamerError>::new(VideoStreamerError(error_msg))
            })?;

        Ok(appsrc)
    }
}
