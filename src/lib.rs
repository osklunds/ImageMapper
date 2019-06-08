
extern crate image;

use image::GenericImageView;
use image::FilterType::Gaussian;
use image::jpeg::JPEGEncoder;
use std::fs::File;
use image::ColorType;

pub fn open_compress_and_save_image(source_path: &str, destination_path: &str) {
    let original = image::open(source_path).unwrap();
    let resized = original.resize(700, 700, Gaussian);
    let width = resized.width();
    let height = resized.height();
    let pixels = resized.raw_pixels();

    let mut file = File::create(destination_path).unwrap();
    let mut encoder = JPEGEncoder::new_with_quality(&mut file, 70);
    encoder.encode(&pixels, width, height, ColorType::RGB(8)).unwrap();
}