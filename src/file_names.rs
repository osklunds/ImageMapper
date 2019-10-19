
use std::ffi::OsStr;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;

use exif::Tag;
use unwrap::unwrap;

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

pub fn extension_is_video_extension(extension: &OsStr) -> bool {
    if let Some(extension) = extension.to_str() {
        match extension.to_lowercase().as_str() {
            "mov" => true,
            "avi" => true,
            "mp4" => true,
            "m4v" => true,
            "mpg" => true,
            "mpeg" => true,
            _ => false
        }
    } else {
        false
    }
}

pub fn extension_is_destination_image_extension(extension: &OsStr) -> bool {
    if let Some(extension) = extension.to_str() {
        match extension.to_lowercase().as_str() {
            "jpg" => true,
            _ => false
        }
    } else {
        false
    }
}

pub fn destination_image_name_from_image_path(image_path: &Path) -> String {
    let file_name = unwrap!(image_path.file_name(), "Could not get the file name of \"{}\"", image_path.display());
    let file_name = unwrap!(file_name.to_str(), "Could not convert the file name {:?} to str", file_name);

    let date_time_string = date_time_string_from_image_path(image_path);

    if date_time_string.is_empty() {
        format!("{}.jpg", file_name)
    } else {
        format!("   {} {}.jpg", date_time_string, file_name)
    }
}

// Returns a string of the format "yyyy-mm-dd hh;mm;ss" if the image has an exif date, or "" if it doesn't.
fn date_time_string_from_image_path(image_path: &Path) -> String {
    let file = unwrap!(File::open(image_path), "Could not open the image \"{}\"", image_path.display());
    let reader = exif::Reader::new(&mut BufReader::new(&file));

    if let Ok(r) = reader {
        if let Some(date_time) = r.get_field(Tag::DateTimeOriginal, false) {
            return format!("{}", date_time.value.display_as(Tag::DateTimeOriginal)).replace(":",";");
        }
    }
    "".to_string()
}

pub fn destination_image_name_to_source_image_name(file_name: &str) -> Option<String> {
    let length = file_name.len();
    if length >= 24 && file_name.get(0..3) == Some("   ") {
        let trimmed = unwrap!(file_name.get(23..(length-4)), "Could not trim the str: {}", file_name);
        return Some(trimmed.to_string());
    } else if length >= 4 {
        let trimmed = unwrap!(file_name.get(0..length-4), "Could not trim the str: {}", file_name);
        return Some(trimmed.to_string());
    } else {
        return None;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    const IMAGE_WITH_EXIF: &str = "test_resources/large-with-exif.jpg";
    const IMAGE_WITHOUT_EXIF: &str = "test_resources/large-without-exif.jpg";

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
    fn extension_is_destination_image_extension_is_true_for_destination_image_extension() {
        assert!(extension_is_destination_image_extension(OsStr::new("jpg")));
    }

    #[test]
    fn extension_is_destination_image_extension_is_false_for_non_destination_file_extensions() {
        let extensions = vec!["jpeg", "JPEG", "png", "PNG", "gif", "GIF", "nef", "NEF", "tif", "TIF", "tiff", "TIFF", "m4v", "mp4", "mov", "pdf", "doc"];

        for extension in extensions.iter() {
            assert!(!extension_is_destination_image_extension(OsStr::new(extension)));
        }
    }

    #[test]
    fn extension_is_video_extension_is_true_for_video_extensions() {
        let extensions = vec!["m4v", "M4V", "mp4", "MP4", "mov", "MOV", "mpg", "MPG", "mpeg", "MPEG", "avi", "AVI"];

        for extension in extensions.iter() {
            assert!(extension_is_video_extension(OsStr::new(extension)));
        }
    }

    #[test]
    fn extension_is_video_extension_is_false_for_non_video_extensions() {
        let extensions = vec!["jpg", "doc", "pdf", "png"];

        for extension in extensions.iter() {
            assert!(!extension_is_video_extension(OsStr::new(extension)));
        }
    }

    #[test]
    fn destination_image_name_for_exif_image() {
        let image_path = PathBuf::from(IMAGE_WITH_EXIF);
        let image_name = destination_image_name_from_image_path(&image_path);
        let correct_image_name = "   2010-03-14 11;22;33 large-with-exif.jpg.jpg".to_string();

        assert_eq!(image_name, correct_image_name);
    }

    #[test]
    fn destination_image_name_for_non_exif_image() {
        let image_path = PathBuf::from(IMAGE_WITHOUT_EXIF);
        let image_name = destination_image_name_from_image_path(&image_path);

        let correct_image_name = "large-without-exif.jpg.jpg".to_string();

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
