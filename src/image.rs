
use std::fs::File;
use std::path::Path;

use image::GenericImageView;
use image::FilterType::Gaussian;
use image::jpeg::JPEGEncoder;
use image::ColorType;

#[cfg(not(test))]
pub fn open_compress_and_save_image(source_path: &Path, destination_path: &Path) {
    let original = image::open(source_path).unwrap();
    let resized = original.resize(1024, 1024, Gaussian);
    let width = resized.width();
    let height = resized.height();
    let pixels = resized.raw_pixels();

    let mut file = File::create(destination_path).unwrap();
    let mut encoder = JPEGEncoder::new_with_quality(&mut file, 60);
    encoder.encode(&pixels, width, height, ColorType::RGB(8)).unwrap();
}

#[cfg(test)]
pub fn open_compress_and_save_image(source_path: &Path, destination_path: &Path) {
    fs::copy(source_path, destination_path).unwrap();
}