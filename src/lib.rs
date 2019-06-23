
#![allow(dead_code, unused_variables, unused_imports)]

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

pub fn map_directory(source_path: &Path, destination_path: &Path) {
    println!("Entered src '{:?}' and dst '{:?}'", source_path, destination_path);

    ensure_destination_path_is_directory(destination_path);

    for source_entry in fs::read_dir(source_path).unwrap() {
        let source_entry = source_entry.unwrap();
        let source_entry_path: PathBuf = source_entry.path();
        let source_entry_name: &Path = source_entry_path.strip_prefix(source_path).unwrap();

        if source_entry_path.is_dir() {
            let destination_entry_path: PathBuf = destination_path.join(source_entry_name);

            map_directory(&source_entry_path, &destination_entry_path);
        } else {
            let extension = source_entry_path.extension();

            if let Some(extension) = extension {
                if extension_is_image_extension(extension) {
                    let destination_entry_name = destination_file_name_from_image_path(&source_entry_path);
                    println!("{:?}", destination_entry_name);
                    let destination_entry_path: PathBuf = destination_path.join(destination_entry_name);

                    if !destination_entry_path.exists() {
                        open_compress_and_save_image(&source_entry_path, &destination_entry_path);
                    }
                }
            }
        }
    }
    
    for destination_entry in fs::read_dir(destination_path).unwrap() {
        let destination_entry = destination_entry.unwrap();
        let destination_entry_path: PathBuf = destination_entry.path();
        let destination_entry_name: &Path = destination_entry_path.strip_prefix(destination_path).unwrap();

        if destination_entry_path.is_dir() {
            let source_entry_path = source_path.join(destination_entry_name);

            if !source_entry_path.is_dir() {
                fs::remove_dir_all(destination_entry_path).unwrap();
            }
        } else {
            let extension = destination_entry_path.extension();

            if let Some(extension) = extension {
                if extension_is_destination_format_extension(extension) {
                    let source_entry_name: String = destination_file_name_to_file_name(destination_entry_name.to_str().unwrap());
                    
                    let source_entry_path = source_path.join(source_entry_name);

                    if !source_entry_path.is_file() {
                        fs::remove_file(destination_entry_path).unwrap();
                    }
                    
                } else if extension_is_image_extension(extension) {
                    let source_entry_path = source_path.join(destination_entry_name);

                    if !source_entry_path.is_file() {
                        println!("Want to delete2 {:?}", destination_entry_path);
                        fs::remove_file(destination_entry_path).unwrap();
                    }
                }
            } else {
                println!("Want to delete3 {:?}", destination_entry_path);
                fs::remove_file(destination_entry_path).unwrap();
            }
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

pub fn extension_is_image_extension(extension: &OsStr) -> bool {
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

pub fn extension_is_destination_format_extension(extension: &OsStr) -> bool {
    let string = extension.to_str().unwrap().to_lowercase();

    return string.as_str() == "jpg";
}

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

pub fn destination_file_name_from_image_path(image_path: &Path) -> String {
    let file_name = image_path.file_name().unwrap();
    let date_time_string = date_time_string_from_image_path(image_path);
    if date_time_string.is_empty() {
        return String::from(file_name.to_str().unwrap());
    } else {
        return format!("   {} {}.jpg", date_time_string, file_name.to_str().unwrap());
    }
}

// Returns a string of the format "yyyy-mm-dd hh;mm;ss" if the image has an exif date, or "" if it doesn't.
pub fn date_time_string_from_image_path(image_path: &Path) -> String {
    let file = std::fs::File::open(image_path).unwrap();
    let reader = exif::Reader::new(&mut std::io::BufReader::new(&file));

    if let Ok(r) = reader {
        let date_time = r.get_field(Tag::DateTimeOriginal, false).unwrap();
        return format!("{}", date_time.value.display_as(Tag::DateTimeOriginal)).replace(":",";");
    } else {
        return String::from("");
    }
}

pub fn destination_file_name_to_file_name(file_name: &str) -> String {
    let length = file_name.len();
    if length < 24 {
        return String::from(file_name);
    } else {
        let x = file_name.get(23..(length-4)).unwrap();
        return x.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_time_string_is_correct() {
        let image_path = PathBuf::from(r"tests/test_image.jpg");
        let date_time_string = date_time_string_from_image_path(&image_path);

        assert_eq!(date_time_string,"2004-04-09 17;33;15");
    }

    #[test]
    fn is_image_extension_is_true_for_image_extensions() {
        let extensions = vec!["jpg", "JPG", "jpeg", "JPEG", "png", "PNG", "gif", "GIF", "nef", "NEF", "tif", "TIF", "tiff", "TIFF"];

        for extension in extensions.iter() {
            assert!(extension_is_image_extension(OsString::from(extension).as_os_str()));
        }
    }

    #[test]
    fn is_image_extension_is_false_for_non_image_extensions() {
        let extensions = vec!["m4v", "mp4", "mov", "pdf", "doc"];

        for extension in extensions.iter() {
            assert!(!extension_is_image_extension(OsString::from(extension).as_os_str()));
        }
    }

    #[test]
    fn test_ensure_destination_path_is_directory_removes_file() {
        let destination_path = Path::new("tests/test_ensure_destination_path_is_directory_removes_file");
        fs::remove_dir(destination_path).unwrap();
        File::create(destination_path).unwrap();

        ensure_destination_path_is_directory(destination_path);

        assert!(destination_path.is_dir());
    }

    #[test]
    fn test_ensure_destination_path_is_directory_adds_directory() {
        let destination_path = Path::new("tests/test_ensure_destination_path_is_directory_adds_directory");
        fs::remove_dir(destination_path).unwrap();

        ensure_destination_path_is_directory(destination_path);

        assert!(destination_path.is_dir());
    }

    #[test]
    fn test_ensure_destination_path_is_directory_does_not_remove_directory() {
        let destination_path = Path::new("tests/test_ensure_destination_path_is_directory_does_not_remove_directory");
        fs::remove_dir_all(destination_path).unwrap();
        fs::create_dir(destination_path).unwrap();

        let destination_file = destination_path.join("file");
        File::create(destination_file.as_path()).unwrap();

        ensure_destination_path_is_directory(destination_path);

        assert!(destination_file.as_path().exists());        
    }

    #[test]
    fn test_destination_file_name_to_file_name() {
        let destination_path = PathBuf::from(r"tests/test_image.jpg");
        let destination_file_name = destination_file_name_from_image_path(&destination_path);
        let file_name = destination_file_name_to_file_name(&destination_file_name);
        assert_eq!("test_image.jpg", file_name);
    }



}
