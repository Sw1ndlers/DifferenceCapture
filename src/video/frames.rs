use crate::methods::method_trait::CalculationMethod;
use crate::utils::types::CodecString;
use crate::utils::{color::Color, pixels::pixel_from_position};
use crate::video::extractor::get_video_frames;
use crate::video::image_builder::ImageBuilder;

use std::hash::Hash;
use std::{
    collections::HashMap,
    io::{stdout, Write},
    sync::{Arc, Mutex},
    thread,
};

use opencv::{
    core::{self},
    prelude::*,
    videoio,
};
use termion::cursor;

use rayon::prelude::*;

const COLOR_BLACK: Color = Color { r: 0, g: 0, b: 0 };

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn get_frame_difference(
    frame_size: &core::Size,
    previous_frame: &Mat,
    current_frame: &Mat,
    threshold: f32,
    method: CalculationMethod,
) -> Mat {
    let mut image_difference =
        ImageBuilder::new(frame_size.width, frame_size.height, previous_frame.typ());

    for y in 0..frame_size.height {
        for x in 0..frame_size.width {
            let previous_pixel = pixel_from_position(previous_frame, x, y);
            let current_pixel = pixel_from_position(current_frame, x, y);

            let previous_color = Color::from_vecn(&previous_pixel);
            let current_color = Color::from_vecn(&current_pixel);

            let distance = previous_color.distance_from(&current_color);

            let color =
                (method.function)(threshold, distance, &current_color, &previous_color);

            image_difference.append_pixel(x, y, color);
        }
    }

    image_difference.image
}


fn convert_frames_to_video(
    output_path: &str,
    frame_size: core::Size,
    frame_differences: HashMap<i32, Mat>,
    output_fps: f64,
    codec: i32,
) -> Result<(), opencv::Error> {
    let _total_frames = frame_differences.len();

    let mut video_writer =
        videoio::VideoWriter::new(output_path, codec, output_fps, frame_size, true)?;

    if !video_writer.is_opened()? {
        return Err(
            opencv::Error::new(core::StsError, "Failed to open video writer").into(),
        );
    }

    let mut i: usize = 0;
    let total_frames = frame_differences.len();

    while i < total_frames {
        let frame = &frame_differences.get(&(i as i32));
        if let Some(frame) = frame {
            video_writer.write(frame)?;
        }

        i += 1;

        print!("{}/{} frames processed\r", i, total_frames);
        stdout().flush().unwrap();
    }

    video_writer.release()?;

    Ok(())
}

fn get_frame_differences(
    video_frames: Vec<Mat>,
    threshold: f32,
    only_compare_first: bool,
    method: CalculationMethod,
    frame_skip: usize,
) -> anyhow::Result<HashMap<i32, Mat>> {
    let last_frame = &video_frames[0];
    let frame_size = last_frame.size()?;

    let frame_differences = Arc::new(Mutex::new(HashMap::new()));
    let frame_differences_ref = Arc::clone(&frame_differences);

    let total_frames = video_frames.len();
    let completed_counter = Arc::new(Mutex::new(0));

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(30)
        .build()
        .unwrap();

    pool.install(|| {
        rayon::scope(move |s| {
            for (i, current_frame) in video_frames.iter().enumerate() {
                let previous_frame = match i {
                    0 => current_frame.clone(),
                    _ => video_frames[i - 1].clone(),
                };

                let frame_differences = Arc::clone(&frame_differences_ref);
                let completed_counter = Arc::clone(&completed_counter);

                let current_frame = current_frame.clone();

                s.spawn(move |_| {
                    let frame_difference = get_frame_difference(
                        &frame_size,
                        &previous_frame,
                        &current_frame,
                        threshold,
                        method,
                    );

                    let mut mutable_frame_differences = frame_differences.lock().unwrap();
                    mutable_frame_differences.insert(i as i32, frame_difference);

                    drop(mutable_frame_differences);

                    let mut mutable_completed_counter = completed_counter.lock().unwrap();
                    *mutable_completed_counter += 1;

                    print!(
                        "{}{}/{} frames processed\r",
                        cursor::Hide,
                        *mutable_completed_counter,
                        total_frames
                    );
                    stdout().flush().unwrap();
                });
            }
        })
    });

    let frame_differences = Arc::try_unwrap(frame_differences)
        .unwrap()
        .into_inner()
        .unwrap();

    Ok(frame_differences)
}


fn test_codec(codec: CodecString) -> anyhow::Result<i32> {
    let codec_result = videoio::VideoWriter::fourcc(codec.0, codec.1, codec.2, codec.3);

    match codec_result {
        Ok(codec) => Ok(codec),
        Err(err) => Err(err.into()),
    }
}

pub fn capture_differences(
    path: &str,
    output_path: &str,
    threshold: f32,
    export_codec: CodecString,
    method: CalculationMethod,
    frame_skip: usize,
) -> anyhow::Result<()> {
    let video_info = get_video_frames(path).expect("Failed to get video frames");
    let codec = test_codec(export_codec).expect("Invalid codec passed");
    let only_compare_first = method.only_compare_first;

    let start = std::time::Instant::now();

    let video_frames = video_info.frames;
    let video_fps = video_info.fps;

    let frame_size = video_frames[0].size()?;

    println!(
        "Got {} video frames ({}x{}) at {} fps\n",
        video_frames.len(),
        frame_size.width,
        frame_size.height,
        video_fps
    );

    println!("Processing frame differences");
    let frame_differences = get_frame_differences(
        video_frames,
        threshold,
        only_compare_first,
        method,
        frame_skip,
    )?;

    println!("\nProcessed frame differences in {:?}", start.elapsed());

    let writing_start = std::time::Instant::now();

    println!("\nWriting {} frames to video", frame_differences.len());

    convert_frames_to_video(
        output_path,
        frame_size,
        frame_differences,
        video_fps,
        codec,
    )?;

    println!("\nWrote video in {:?}", writing_start.elapsed());
    println!("\nCompleted in: {:?}", start.elapsed());

    Ok(())
}
