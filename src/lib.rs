
#![allow(dead_code, unused_variables, unused_imports)]

extern crate image;

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

mod file_names;

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
    let source_entries = match open_dir_to_iterator(source_path) {
        Some(it) => it,
        None => return ()
    };

    for source_entry in source_entries {
        let source_entry = match open_dir_entry(source_entry) {
            Some(se) => se,
            None => continue
        };

        let source_entry_path = &source_entry.path();

        if source_entry_path.is_dir() {
            handle_source_dir(source_entry_path, destination_path);
        } else {
            handle_source_file(source_entry_path, destination_path);
        }
    }
}

fn open_dir_to_iterator(path: &Path) -> Option<ReadDir> {
    match fs::read_dir(path) {
        Ok(it) => Some(it),
        Err(e) => {
            println!("Could not open \"{}\": {}", path.canonicalize().unwrap().display(), e);
            return None;
        }
    }
}

fn open_dir_entry(dir_entry: io::Result<DirEntry>) -> Option<DirEntry> {
    match dir_entry {
        Ok(dir_entry) => Some(dir_entry),
        Err(e) => {
            println!("Error with an entry: {}", e);
            None
        }
    }
}

fn handle_source_dir(source_dir_path: &Path, destination_path: &Path) {
    let source_dir_name = source_dir_path.file_name().expect("Could not get the file name.");
    let destination_dir_path = &destination_path.join(source_dir_name);

    map_directory(source_dir_path, destination_dir_path);
}

fn handle_source_file(source_file_path: &Path, destination_path: &Path) {
    let extension: Option<&OsStr> = source_file_path.extension();

    if let Some(extension) = extension {
        if file_names::extension_is_image_extension(extension) {
            handle_source_image(source_file_path, destination_path);
        }
    }
}

fn handle_source_image(source_image_path: &Path, destination_path: &Path) {
    if let Some(destination_image_name) = file_names::destination_image_name_from_image_path(source_image_path) {
        println!("{:?}", destination_image_name);
        let destination_image_path: PathBuf = destination_path.join(destination_image_name);

        if !destination_image_path.exists() {
            open_compress_and_save_image(source_image_path, &destination_image_path);
        }
    } else {
        //TODO
    }
}

fn iterate_destination_entries(source_path: &Path, destination_path: &Path) {
    let destination_entries = match open_dir_to_iterator(destination_path) {
        Some(iterator) => iterator,
        None           => return ()
    };

    for destination_entry in destination_entries {
        let destination_entry = match open_dir_entry(destination_entry) {
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
        if file_names::extension_is_destination_file_extension(extension) {
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
        let corresponding_source_entry_name = file_names::destination_image_name_to_source_image_name(destination_image_name.to_str().unwrap()).unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    const TESTING_ROOT_DIRECTORY: &str = "testing/temp";

    #[test]
    fn test_ensure_path_is_directory_removes_file() {
        clean_testing_directory();
        let destination_path = Path::new("testing/temp/dst");
        File::create(destination_path).unwrap();

        ensure_path_is_directory(destination_path);

        assert!(destination_path.is_dir());
    }

    fn clean_testing_directory() {
        fs::remove_dir_all(TESTING_ROOT_DIRECTORY).unwrap();
        fs::create_dir(TESTING_ROOT_DIRECTORY).unwrap();
    }

    #[test]
    fn test_ensure_path_is_directory_adds_directory() {
        clean_testing_directory();
        let destination_path = Path::new("testing/temp/dst");
        ensure_path_is_directory(destination_path);

        assert!(destination_path.is_dir());
    }
    
    #[test]
    fn test_ensure_path_is_directory_does_not_remove_directory() {
        clean_testing_directory();
        let destination_path = Path::new("testing/temp/dst");
        fs::create_dir(destination_path).unwrap();

        let destination_file = destination_path.join("file");
        File::create(destination_file.as_path()).unwrap();

        ensure_path_is_directory(destination_path);

        assert!(destination_file.as_path().exists());        
    }
}
