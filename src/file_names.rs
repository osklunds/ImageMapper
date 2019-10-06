
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

pub fn destination_image_name_to_source_image_name(file_name: &str) -> Option<String> {
    let length = file_name.len();
    if length >= 24 && file_name.get(0..3) == Some("   ") {
        let trimmed = file_name.get(23..(length-4)).expect("Could not trim a file name.");
        return Some(trimmed.to_string());
    } else if length >= 4 {
        return Some(file_name.get(0..length-4).expect("Could not trim a file name.").to_string());
    } else {
        return None;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const IMAGE_WITH_EXIF: &str = "testing/images/with-exif.jpg";
    const IMAGE_WITHOUT_EXIF: &str = "testing/images/without-exif.jpg";

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
        let image_path = PathBuf::from(IMAGE_WITH_EXIF);
        let image_name = destination_image_name_from_image_path(&image_path);
        let correct_image_name = Some(String::from("   2010-03-14 11;22;33 with-exif.jpg.jpg"));

        assert_eq!(image_name, correct_image_name);
    }

    #[test]
    fn destination_image_name_for_non_exif_image() {
        let image_path = PathBuf::from(IMAGE_WITHOUT_EXIF);
        let image_name = destination_image_name_from_image_path(&image_path);

        let correct_image_name = Some(String::from("without-exif.jpg.jpg"));

        assert_eq!(image_name, correct_image_name);
    }

    #[test]
    fn date_time_string_is_correct_for_image_with_exif() {
        let image_path = PathBuf::from(IMAGE_WITH_EXIF);
        let date_time_string = date_time_string_from_image_path(&image_path);

        assert_eq!(date_time_string,"2010-03-14 11;22;33");
    }

    #[test]
    fn date_time_string_is_correct_for_image_without_exif() {
        let image_path = PathBuf::from(IMAGE_WITHOUT_EXIF);
        let date_time_string = date_time_string_from_image_path(&image_path);

        assert_eq!(date_time_string,"");
    }
    
    #[test]
    fn test_destination_image_name_to_source_image_name_exif() {
        let destination_image_name = "   2019-02-01 11;22;33 image.png.jpg";
        let source_image_name = destination_image_name_to_source_image_name(destination_image_name);
        let expected_source_image_name = Some("image.png".to_string());
        assert_eq!(source_image_name, expected_source_image_name);
    }

    #[test]
    fn test_destination_image_name_to_source_image_name_no_exif() {
        let destination_image_name = "image.png.jpg";
        let source_image_name = destination_image_name_to_source_image_name(destination_image_name);
        let expected_source_image_name = Some("image.png".to_string());
        assert_eq!(source_image_name, expected_source_image_name);
    }

    #[test]
    fn test_destination_image_name_to_source_image_name_no_exif_long_name() {
        let destination_image_name = "image image image image image image image.png.jpg";
        let source_image_name = destination_image_name_to_source_image_name(destination_image_name);
        let expected_source_image_name = Some("image image image image image image image.png".to_string());
        assert_eq!(source_image_name, expected_source_image_name);
    }

    #[test]
    fn test_destination_image_name_to_source_image_name_too_short() {
        let destination_image_name = "b";
        let source_image_name = destination_image_name_to_source_image_name(destination_image_name);
        assert_eq!(source_image_name, None);
    }
}
