
extern crate exif;

use exif::Tag;

use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::PathBuf;
use std::path::Path;

use std::fs::File;
use std::fs;
use std::fs::ReadDir;
use std::fs::DirEntry;

pub fn extension_is_image_extension(extension: &OsStr) -> bool {
    if let Some(extension) = extension.to_str() {
        match extension.to_lowercase().as_str() {
            "jpg" => true,
            "jpeg" => true,
            "png" => true,
            "gif" => true,
            "nef" => true,
            "tif" => true,
            "tiff" => true,
            _ => false
        }
    } else {
        false
    }
}

pub fn extension_is_destination_file_extension(extension: &OsStr) -> bool {
    if let Some(extension) = extension.to_str() {
        match extension.to_lowercase().as_str() {
            "jpg" => true,
            _ => false
        }
    } else {
        false
    }
}

pub fn destination_image_name_from_image_path(image_path: &Path) -> Option<String> {
    let file_name = image_path.file_name()?.to_str()?;
    let date_time_string = date_time_string_from_image_path(image_path);

    if date_time_string.is_empty() {
        Some(format!("{}.jpg", file_name))
    } else {
        Some(format!("   {} {}.jpg", date_time_string, file_name))
    }
}

// Returns a string of the format "yyyy-mm-dd hh;mm;ss" if the image has an exif date, or "" if it doesn't.
fn date_time_string_from_image_path(image_path: &Path) -> String {
    let file = std::fs::File::open(image_path).unwrap();
    let reader = exif::Reader::new(&mut std::io::BufReader::new(&file));

    if let Ok(r) = reader {
        let date_time = r.get_field(Tag::DateTimeOriginal, false).unwrap();
        return format!("{}", date_time.value.display_as(Tag::DateTimeOriginal)).replace(":",";");
    } else {
        return String::from("");
    }
}

pub fn destination_image_name_to_source_image_name(file_name: &str) -> String {
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
    fn extension_is_image_extension_is_true_for_image_extensions() {
        let extensions = vec!["jpg", "JPG", "jpeg", "JPEG", "png", "PNG", "gif", "GIF", "nef", "NEF", "tif", "TIF", "tiff", "TIFF"];

        for extension in extensions.iter() {
            assert!(extension_is_image_extension(OsStr::new(extension)));
        }
    }
    
    #[test]
    fn extension_is_image_extension_is_false_for_non_image_extensions() {
        let extensions = vec!["m4v", "mp4", "mov", "pdf", "doc"];

        for extension in extensions.iter() {
            assert!(!extension_is_image_extension(OsStr::new(extension)));
        }
    }

    #[test]
    fn extension_is_destination_file_extension_is_true_for_destination_file_extension() {
        assert!(extension_is_destination_file_extension(OsStr::new("jpg")));
    }

    #[test]
    fn extension_is_destination_file_extension_is_false_for_non_destination_file_extensions() {
        let extensions = vec!["jpeg", "JPEG", "png", "PNG", "gif", "GIF", "nef", "NEF", "tif", "TIF", "tiff", "TIFF", "m4v", "mp4", "mov", "pdf", "doc"];

        for extension in extensions.iter() {
            assert!(!extension_is_destination_file_extension(OsStr::new(extension)));
        }
    }

    #[test]
    fn destination_image_name_for_exif_image() {
        let image_path = PathBuf::from(r"tests/image_with_exif.jpg");
        let image_name = destination_image_name_from_image_path(&image_path);
        let correct_image_name = Some(String::from("   2004-04-09 17;33;15 image_with_exif.jpg.jpg"));

        assert_eq!(image_name, correct_image_name);
    }

    #[test]
    fn destination_image_name_for_non_exif_image() {
        let image_path = PathBuf::from(r"tests/image_without_exif.jpg");
        let image_name = destination_image_name_from_image_path(&image_path);

        let correct_image_name = Some(String::from("image_without_exif.jpg.jpg"));

        assert_eq!(image_name, correct_image_name);
    }

    #[test]
    fn date_time_string_is_correct_for_image_with_exif() {
        let image_path = PathBuf::from(r"tests/image_with_exif.jpg");
        let date_time_string = date_time_string_from_image_path(&image_path);

        assert_eq!(date_time_string,"2004-04-09 17;33;15");
    }

    #[test]
    fn date_time_string_is_correct_for_image_without_exif() {
        let image_path = PathBuf::from(r"tests/image_without_exif.jpg");
        let date_time_string = date_time_string_from_image_path(&image_path);

        assert_eq!(date_time_string,"");
    }
    
    /*
    #[test]
    fn test_destination_file_name_to_file_name() {
        let destination_path = PathBuf::from(r"tests/test_image.jpg");
        let destination_file_name = destination_file_name_from_image_path(&destination_path);
        let file_name = destination_file_name_to_file_name(&destination_file_name);
        assert_eq!("test_image.jpg", file_name);
    }
    */
}
