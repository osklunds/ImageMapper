
use std::fs::File;
use std::path::Path;
use std::io::BufReader;

use image::GenericImageView;
use image::FilterType::Gaussian;
use image::jpeg::JPEGEncoder;
use image::ColorType;
use image::DynamicImage;
use unwrap::unwrap;
use exif::{Reader, Tag, Value};

use crate::settings::{Settings, ImageQuality};

#[cfg(not(test))]
pub fn open_compress_and_save_image(source_path: &Path, destination_path: &Path, settings: &Settings) {
    let original = read_original_image(source_path);
    let orientation = orientation_from_path(source_path);
    let rotated = rotate_image(original, orientation);
    let dimensions = dimensions_from_settings(settings);
    let resized = resize_image(rotated, dimensions);
    encode_and_save_image(resized, destination_path, settings);
}

fn read_original_image(image_path: &Path) -> DynamicImage {
    unwrap!(image::open(image_path), "Could not open the image {:?}", image_path)
}

fn orientation_from_path(image_path: &Path) -> u16 {
    let file = unwrap!(File::open(image_path), "Could not open the image for exif {:?}", image_path);
    let mut buf_reader = BufReader::new(&file);
    let exif_reader = Reader::new(&mut buf_reader);

    if let Ok(exif_reader) = exif_reader {
        if let Some(orientation) = exif_reader.get_field(Tag::Orientation, false) {
            if let Value::Short(orientation) = &orientation.value {
                if orientation.len() == 1 {
                    return orientation[0];
                }
            }
        }
    }

    1
}

fn rotate_image(image: DynamicImage, orientation: u16) -> DynamicImage {
    match orientation {
        1 => image,
        6 => image.rotate90(),
        8 => image.rotate270(),
        3 => image.rotate180(),
        _ => panic!("Unsupported orientation")
    }
}

fn dimensions_from_settings(settings: &Settings) -> (u32, u32) {
    match settings.image_quality {
        ImageQuality::Mobile => (1024, 1024),
        ImageQuality::Television => (1920, 1080),
    }
}

fn resize_image(image: DynamicImage, dimensions: (u32, u32)) -> DynamicImage {
    let (width, height) = dimensions;
    image.resize(width, height, Gaussian)
}

fn encode_and_save_image(image: DynamicImage, destination_path: &Path, settings: &Settings) {
    let color = image.color();
    let width = image.width();
    let height = image.height();
    let pixels = image.raw_pixels();

    let mut file = unwrap!(File::create(destination_path), "Could not create the image {:?}", destination_path);
    let factor = match settings.image_quality {
        ImageQuality::Mobile     => 30,
        ImageQuality::Television => 70,
    };

    let mut encoder = JPEGEncoder::new_with_quality(&mut file, factor);
    unwrap!(encoder.encode(&pixels, width, height, color), "Could not encode the image {:?}", destination_path);
}

#[cfg(test)]
pub fn open_compress_and_save_image(source_path: &Path, destination_path: &Path, settings: &Settings) {
    use std::fs;
    unwrap!(fs::copy(source_path, destination_path), "Could not copy from {:?} to {:?}", source_path, destination_path);
}
