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

use methods::method_trait::{CalculationMethod, CalculationMethodTrait};
use utils::color::Color;
use video::frames::capture_differences;

const COLOR_BLACK: Color = Color { r: 0, g: 0, b: 0 };

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

create_method!(
    GreyscaleDistance,
    |threshold: f32, distance: f32, current_color: &Color, previous_color: &Color| {
        if distance > threshold {
            return Color::new(distance as u8, distance as u8, distance as u8);
        }
        return COLOR_BLACK;
    },
    false
);

create_method!(
    Contrast,
    |threshold: f32, distance: f32, current_color: &Color, previous_color: &Color| {
        if distance > threshold {
            return current_color.multiply(1.1);
        }
        return previous_color.multiply(0.1);
    },
    false
);

fn main() -> anyhow::Result<()> {
    let path = "assets/stars.mp4";
    let output_path = "./assets/output.mp4";

    let threshold = 60.0;
    let frame_skip = 1;
    let export_codec = ('a', 'v', 'c', '1');

    let method = BlackStill::get_method();

    capture_differences(
        path,
        output_path,
        threshold,
        export_codec,
        method,
        frame_skip,
    )
    .unwrap();

    Ok(())
}
