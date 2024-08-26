#![allow(dead_code)]

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use exif::{Reader, Tag, Value, In};
use image::codecs::jpeg::JpegEncoder;
use image::imageops::Gaussian;
use image::DynamicImage;
use unwrap::unwrap;

use crate::settings::{ImageQuality, Settings};

pub fn open_compress_and_save_image(
    source_path: &Path,
    destination_path: &Path,
    settings: &Settings,
) -> bool {
    if let Some(original) = read_original_image(source_path) {
        let orientation = orientation_from_path(source_path);

        if let Some(rotated) = rotate_image(original, orientation) {
            let dimensions = dimensions_from_settings(settings);
            let resized = resize_image(rotated, dimensions);
            encode_and_save_image(resized, destination_path, settings)
        } else {
            println!(
                "Unsupported orientation for \"{}\"",
                source_path.display()
            );
            false
        }
    } else {
        false
    }
}

fn read_original_image(image_path: &Path) -> Option<DynamicImage> {
    match image::open(image_path) {
        Ok(image) => Some(image),
        Err(e) => {
            println!(
                "Could not open the image \"{}\" due to \"{}\", so skipping it.",
                image_path.display(),
                e
            );
            None
        }
    }
}

fn orientation_from_path(image_path: &Path) -> u16 {
    let file = unwrap!(
        File::open(image_path),
        "Could not open the image for exif \"{}\"",
        image_path.display()
    );
    let mut buf_reader = BufReader::new(&file);
    let exif_reader = Reader::new();

    if let Ok(exif_reader) = exif_reader.read_from_container(&mut buf_reader) {
        if let Some(orientation) =
            exif_reader.get_field(Tag::Orientation, In::PRIMARY)
        {
            if let Value::Short(orientation) = &orientation.value {
                if orientation.len() == 1 {
                    return orientation[0];
                }
            }
        }
    }

    1
}

fn rotate_image(image: DynamicImage, orientation: u16) -> Option<DynamicImage> {
    match orientation {
        1 => Some(image),
        2 => Some(image.fliph()),
        3 => Some(image.rotate180()),
        4 => Some(image.rotate180().fliph()),
        5 => Some(image.rotate90().fliph()),
        6 => Some(image.rotate90()),
        7 => Some(image.rotate270().fliph()),
        8 => Some(image.rotate270()),
        _ => None,
    }
}

fn dimensions_from_settings(settings: &Settings) -> (u32, u32) {
    match settings.image_quality {
        ImageQuality::Mobile => (1024, 1024),
        ImageQuality::Television => (1920, 1080),
        ImageQuality::Thumbnail => (300, 300),
    }
}

fn resize_image(image: DynamicImage, dimensions: (u32, u32)) -> DynamicImage {
    let (width, height) = dimensions;
    image.resize(width, height, Gaussian)
}

fn encode_and_save_image(
    image: DynamicImage,
    destination_path: &Path,
    settings: &Settings,
) -> bool {
    let color = image.color();
    let width = image.width();
    let height = image.height();
    let pixels = image.as_bytes();

    let mut file = unwrap!(
        File::create(destination_path),
        "Could not create the image \"{}\"",
        destination_path.display()
    );
    let factor = match settings.image_quality {
        ImageQuality::Mobile => 30,
        ImageQuality::Television => 70,
        ImageQuality::Thumbnail => 30,
    };

    let mut encoder = JpegEncoder::new_with_quality(&mut file, factor);
    match encoder.encode(&pixels, width, height, color) {
        Ok(()) => true,
        Err(e) => {
            println!(
                "Could not convert the image \"{}\". Error \"{}\"",
                destination_path.display(),
                e
            );
            let _ = std::fs::remove_file(destination_path);
            false
        }
    }
}
