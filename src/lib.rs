
extern crate image;
extern crate exif;

use image::GenericImageView;
use image::FilterType::Gaussian;
use image::jpeg::JPEGEncoder;
use image::ColorType;

use std::fs::File;

use exif::Tag;
use exif::DateTime;

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
}