
#![allow(dead_code, unused_variables, unused_imports)]

extern crate image;
extern crate exif;

use image::GenericImageView;
use image::FilterType::Gaussian;
use image::jpeg::JPEGEncoder;
use image::ColorType;

use std::fs::File;
use std::fs;
use std::fs::ReadDir;
use std::fs::DirEntry;

use std::path::PathBuf;
use std::path::Path;

use std::ffi::OsStr;
use std::ffi::OsString;

use std::io;

use exif::Tag;

pub fn map_directory(source_path: &Path, destination_path: &Path) {
    println!("Entered src '{:?}' and dst '{:?}'", source_path, destination_path);

    ensure_path_is_directory(destination_path);

    iterate_source_entries(source_path, destination_path);
    iterate_destination_entries(source_path, destination_path);
}

fn ensure_path_is_directory(destination_path: &Path) {
    if destination_path.is_file() {
        fs::remove_file(destination_path).unwrap();
    }
    if !destination_path.exists() {
        fs::create_dir(destination_path).unwrap();
    }
}

fn iterate_source_entries(source_path: &Path, destination_path: &Path) {
    let source_entries = match read_dir_to_iterator(source_path) {
        Some(it) => it,
        None => return ()
    };

    for source_entry in source_entries {
        let source_entry = match unwrap_dir_entry(source_entry) {
            Some(se) => se,
            None => continue
        };

        let source_entry_path_buf: PathBuf = source_entry.path();
        let source_entry_path: &Path = &source_entry_path_buf;

        if source_entry_path.is_dir() {
            handle_source_dir(source_entry_path, source_path, destination_path);
        } else {
            handle_source_file(source_entry_path, destination_path);
        }
    }
}

fn read_dir_to_iterator(path: &Path) -> Option<ReadDir> {
    match fs::read_dir(path) {
        Ok(it) => Some(it),
        Err(e) => {
            println!("Could not open \"{}\": {}", path.canonicalize().unwrap().display(), e);
            return None;
        }
    }
}

fn unwrap_dir_entry(dir_entry: io::Result<DirEntry>) -> Option<DirEntry> {
    match dir_entry {
        Ok(dir_entry) => Some(dir_entry),
        Err(e) => {
            println!("Error with an entry: {}", e);
            None
        }
    }
}

// Important: source_path must be a super path of source_dir_path.
fn handle_source_dir(source_dir_path: &Path, source_path: &Path, destination_path: &Path) {
    let source_dir_name: &Path = source_dir_path.strip_prefix(source_path).unwrap();
    let destination_dir_path: PathBuf = destination_path.join(source_dir_name);

    map_directory(source_dir_path, &destination_dir_path);
}

fn handle_source_file(source_file_path: &Path, destination_path: &Path) {
    let extension: Option<&OsStr> = source_file_path.extension();

    if let Some(extension) = extension {
        if extension_is_image_extension(extension) {
            handle_source_image(source_file_path, destination_path);
        }
    }
}

fn handle_source_image(source_image_path: &Path, destination_path: &Path) {
    let destination_image_name: String = destination_image_name_from_image_path(source_image_path);
    println!("{:?}", destination_image_name);
    let destination_image_path: PathBuf = destination_path.join(destination_image_name);

    if !destination_image_path.exists() {
        open_compress_and_save_image(source_image_path, &destination_image_path);
    }
}

fn iterate_destination_entries(source_path: &Path, destination_path: &Path) {
    let destination_entries = match read_dir_to_iterator(destination_path) {
        Some(iterator) => iterator,
        None           => return ()
    };

    for destination_entry in destination_entries {
        let destination_entry = match unwrap_dir_entry(destination_entry) {
            Some(de) => de,
            None     => return ()
        };

        let destination_entry_path_buf = destination_entry.path();
        let destination_entry_path = &destination_entry_path_buf;

        if destination_entry_path.is_dir() {
            handle_destination_dir(destination_entry_path, source_path);
        } else {
            handle_destination_file(destination_entry_path, source_path);
        }
    }
}

fn handle_destination_dir(destination_dir_path: &Path, source_path: &Path) {
    let destination_dir_name = destination_dir_path.file_name();

    if let Some(destination_dir_name) = destination_dir_name {
        let corresponding_source_entry_path = source_path.join(destination_dir_name);

        if !corresponding_source_entry_path.is_dir() {
            fs::remove_dir_all(destination_dir_path).unwrap();
        }
    } else {
        //TODO
    }
}

fn handle_destination_file(destination_file_path: &Path, source_path: &Path) {
    let extension = destination_file_path.extension();

    if let Some(extension) = extension {
        if extension_is_destination_file_extension(extension) {
            handle_destination_image(destination_file_path, source_path);
        } else {
            handle_destination_non_image_file(destination_file_path);
        }
    } else {
        handle_destination_extensionless_file(destination_file_path);
    }
}

fn handle_destination_image(destination_image_path: &Path,source_path: &Path) {
    let destination_image_name = destination_image_path.file_name();

    if let Some(destination_image_name) = destination_image_name {
        let corresponding_source_entry_name: String = destination_image_name_to_source_image_name(destination_image_name.to_str().unwrap());
        let corresponding_source_entry_path = source_path.join(corresponding_source_entry_name);

        if !corresponding_source_entry_path.is_file() {
            fs::remove_file(destination_image_path).unwrap();
        }

    } else {
        //TODO
    }
}

fn handle_destination_non_image_file(destination_file_path: &Path) {
    fs::remove_file(destination_file_path).unwrap();
}

fn handle_destination_extensionless_file(destination_file_path: &Path) {
    fs::remove_file(destination_file_path).unwrap();
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

pub fn extension_is_destination_file_extension(extension: &OsStr) -> bool {
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

pub fn destination_image_name_from_image_path(image_path: &Path) -> String {
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
    fn test_ensure_path_is_directory_removes_file() {
        let destination_path = Path::new("tests/test_ensure_destination_path_is_directory_removes_file");
        fs::remove_dir(destination_path).unwrap();
        File::create(destination_path).unwrap();

        ensure_path_is_directory(destination_path);

        assert!(destination_path.is_dir());
    }

    #[test]
    fn test_ensure_path_is_directory_adds_directory() {
        let destination_path = Path::new("tests/test_ensure_destination_path_is_directory_adds_directory");
        fs::remove_dir(destination_path).unwrap();

        ensure_path_is_directory(destination_path);

        assert!(destination_path.is_dir());
    }

    #[test]
    fn test_ensure_path_is_directory_does_not_remove_directory() {
        let destination_path = Path::new("tests/test_ensure_destination_path_is_directory_does_not_remove_directory");
        fs::remove_dir_all(destination_path).unwrap();
        fs::create_dir(destination_path).unwrap();

        let destination_file = destination_path.join("file");
        File::create(destination_file.as_path()).unwrap();

        ensure_path_is_directory(destination_path);

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
