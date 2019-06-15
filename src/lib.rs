
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

use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;

use exif::Tag;

pub fn map_directory(source: &Path, destination: &Path) {
    println!("Entered src '{}' and dst '{}'", source.display(), destination.display());

    if destination.is_file() {
        fs::remove_file(destination).unwrap();
    }
    if !destination.exists() {
        fs::create_dir(destination).unwrap();
    }



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

fn ensure_destination_path_is_directory(destination_path: &Path) {
    if destination_path.is_file() {
        fs::remove_file(destination_path).unwrap();
    }
    if !destination_path.exists() {
        fs::create_dir(destination_path).unwrap();
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

    #[test]
    fn test_ensure_destination_path_is_directory_removes_file() {
        let destination_path = Path::new("tests/test_ensure_destination_path_is_directory_removes_file");
        fs::remove_dir(destination_path);
        File::create(destination_path);

        ensure_destination_path_is_directory(destination_path);

        assert!(destination_path.is_dir());
    }

    #[test]
    fn test_ensure_destination_path_is_directory_adds_directory() {
        let destination_path = Path::new("tests/test_ensure_destination_path_is_directory_adds_directory");
        fs::remove_dir(destination_path);

        ensure_destination_path_is_directory(destination_path);

        assert!(destination_path.is_dir());
    }

    #[test]
    fn test_ensure_destination_path_is_directory_does_not_remove_directory() {
        let destination_path = Path::new("tests/test_ensure_destination_path_is_directory_does_not_remove_directory");
        fs::remove_dir_all(destination_path);
        fs::create_dir(destination_path);

        let destination_file = destination_path.join("file");
        File::create(destination_file.as_path());

        ensure_destination_path_is_directory(destination_path);

        assert!(destination_file.as_path().exists());        
    }

}
