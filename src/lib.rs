
extern crate image;
extern crate exif;

use image::GenericImageView;
use image::FilterType::Gaussian;
use image::jpeg::JPEGEncoder;
use image::ColorType;

use std::fs::File;
use std::fs;
use std::path::PathBuf;
use std::path::Path;

use std::ffi::OsStr;
use std::ffi::OsString;

use exif::Tag;

pub fn map_directory(source: &Path, _destination: &Path) {
    println!("Directory: {}", source.display());
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            map_directory(&path, &path);
        } else {
            println!("File: {}", path.display());

            let ext = path.extension();
            println!("{:?}", ext);




        }
    }
}

pub fn is_image_extension(extension: &OsStr) -> bool {
    let string = extension.to_str().unwrap().to_lowercase();

    match string.as_str() {
        "jpg" => true,
        "jpeg" => true,
        "png" => true,
        "gif" => true,
        "nef" => true,
        "tif" => true,
        "tiff" => true,
        _ => false
    }
}

pub fn open_compress_and_save_image(source_path: &str, destination_path: &str) {
    let original = image::open(source_path).unwrap();
    let resized = original.resize(1024, 1024, Gaussian);
    let width = resized.width();
    let height = resized.height();
    let pixels = resized.raw_pixels();

    let mut file = File::create(destination_path).unwrap();
    let mut encoder = JPEGEncoder::new_with_quality(&mut file, 60);
    encoder.encode(&pixels, width, height, ColorType::RGB(8)).unwrap();
}

pub fn date_time_string_from_image_path(image_path: &str) -> String {
    let file = std::fs::File::open(image_path).unwrap();
    let reader = exif::Reader::new(&mut std::io::BufReader::new(&file)).unwrap();
    let date_time = reader.get_field(Tag::DateTimeOriginal, false).unwrap();

    return format!("{}", date_time.value.display_as(Tag::DateTimeOriginal));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_time_string_is_correct() {
        let string = date_time_string_from_image_path("tests/test_image.jpg");

        assert_eq!(string.as_str(),"2004-04-09 17:33:15");
    }

    #[test]
    fn is_image_extension_true_positive() {
        let extensions = vec!["jpg", "JPG", "jpeg", "JPEG", "png", "PNG", "gif", "GIF", "nef", "NEF", "tif", "TIF", "tiff", "TIFF"];

        for extension in extensions.iter() {
            assert!(is_image_extension(OsString::from(extension).as_os_str()));
        }
    }

    #[test]
    fn is_image_extension_true_negative() {
        let extensions = vec!["m4v", "mp4", "mov", "pdf", "doc"];

        for extension in extensions.iter() {
            assert!(!is_image_extension(OsString::from(extension).as_os_str()));
        }
    }
}
