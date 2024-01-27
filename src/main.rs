pub mod methods {
    pub mod method_trait;
}

pub mod utils {
    pub mod color;
    pub mod pixels;
    pub mod types;
}

pub mod video {
    pub mod extractor;
    pub mod frames;
    pub mod image_builder;
}

use std::path::PathBuf;

use methods::method_trait::{CalculationMethod, CalculationMethodTrait};
use utils::color::Color;
use video::frames::capture_differences;

const COLOR_BLACK: Color = Color { r: 0, g: 0, b: 0 };

// Dim Still
create_method!(
    DimStill,
    |threshold: f32, distance: f32, current_color: &Color, previous_color: &Color| {
        if distance > threshold {
            return *current_color;
        }
        return previous_color.multiply(0.1);
    },
    false
);

// Black Still
create_method!(
    BlackStill,
    |threshold: f32, distance: f32, current_color: &Color, previous_color: &Color| {
        if distance > threshold {
            return *current_color;
        }
        return COLOR_BLACK;
    },
    false
);

// Greyscale Distance
create_method!(
    GreyscaleDistance,
    |threshold: f32, distance: f32, current_color: &Color, previous_color: &Color| {
        if distance > (threshold / 2.0) {
            return Color::new(distance as u8, distance as u8, distance as u8);
        }
        return COLOR_BLACK;
    },
    false
);


fn main() -> anyhow::Result<()> {
    let threshold = 60.0;
    let frame_skip = 1;
    let export_codec = ('a', 'v', 'c', '1');

    let method = DimStill::get_method();

    let base_video_name = "Waterfall";
    let path = PathBuf::from(format!("examples/waterfall/{}.mp4", base_video_name));
    let output_path = path
        .parent()
        .unwrap()
        .join(format!("{}-{}.mp4", base_video_name, method.name));

    capture_differences(
        path.to_str().unwrap(),
        output_path.to_str().unwrap(),
        threshold,
        export_codec,
        method,
        frame_skip,
    )
    .unwrap();

    Ok(())
}
