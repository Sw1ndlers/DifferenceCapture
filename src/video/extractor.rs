use std::path::Path;

use opencv::{
    core::{self, Mat},
    videoio::{self, VideoCaptureTrait, VideoCaptureTraitConst},
};

pub struct VideoExtraction {
    pub frames: Vec<Mat>,
    pub fps: f64,
}

pub fn get_video_frames(path: &str) -> Result<VideoExtraction, opencv::Error> {
    let mut video_capture = videoio::VideoCapture::from_file(path, videoio::CAP_ANY)?;
    let fps = video_capture.get(videoio::CAP_PROP_FPS)?;

    if !video_capture.is_opened()? {
        if !Path::new(path).exists() {
            return Err(opencv::Error::new(
                core::StsError,
                "Input file does not exist",
            ));
        }

        return Err(opencv::Error::new(
            core::StsError,
            "Failed to open video file",
        ));
    }

    video_capture.set(videoio::CAP_PROP_POS_FRAMES, 45.0)?;

    let mut frames = Vec::new();
    let mut frame = Mat::default();

    while video_capture.read(&mut frame)? {
        frames.push(frame.clone());
    }

    // Ok(frames)
    Ok(VideoExtraction { frames, fps })
}
