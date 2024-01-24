use crate::utils::{color::Color, pixels::mut_pixel_from_position};
use opencv::core::{Mat, Scalar};

#[derive(Debug, Clone)]
pub struct ImageBuilder {
    pub image: Mat,
}

impl ImageBuilder {
    pub fn new(width: i32, height: i32, frame_type: i32) -> Self {
        let image =
            Mat::new_rows_cols_with_default(height, width, frame_type, Scalar::all(0.0))
                .unwrap();

        Self { image }
    }

    pub fn append_pixel(&mut self, x: i32, y: i32, color: Color) {
        let pixel = mut_pixel_from_position(&mut self.image, x, y);

        pixel[0] = color.r;
        pixel[1] = color.g;
        pixel[2] = color.b;
    }
}
