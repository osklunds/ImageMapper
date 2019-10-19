
use std::fs::File;
use std::path::Path;
use std::io::BufReader;

use image::GenericImageView;
use image::FilterType::Gaussian;
use image::jpeg::JPEGEncoder;
use image::ColorType;
use unwrap::unwrap;
use exif::{Reader, Tag, Value};

use crate::settings::{Settings, ImageQuality};

#[cfg(not(test))]
pub fn open_compress_and_save_image(source_path: &Path, destination_path: &Path, settings: &Settings) {
    let original = unwrap!(image::open(source_path), "Could not open the image {:?}", source_path);
    let resized = match settings.image_quality {
        ImageQuality::Mobile     => original.resize(1024, 1024, Gaussian),
        ImageQuality::Television => original.resize(1920, 1080, Gaussian),
    };
        
    let color = resized.color();
    let width = resized.width();
    let height = resized.height();
    let pixels = resized.raw_pixels();

    let mut file = unwrap!(File::create(destination_path), "Could not create the image {:?}", destination_path);
    let factor = match settings.image_quality {
        ImageQuality::Mobile     => 30,
        ImageQuality::Television => 70,
    };
    let mut encoder = JPEGEncoder::new_with_quality(&mut file, factor);
    unwrap!(encoder.encode(&pixels, width, height, color), "Could not encode the image {:?}", destination_path);
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

#[cfg(test)]
pub fn open_compress_and_save_image(source_path: &Path, destination_path: &Path, settings: &Settings) {
    use std::fs;
    unwrap!(fs::copy(source_path, destination_path), "Could not copy from {:?} to {:?}", source_path, destination_path);
}