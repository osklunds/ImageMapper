
#![allow(dead_code, unused_variables, unused_imports)]

use std::fs::File;
use std::fs;
use std::fs::ReadDir;
use std::fs::DirEntry;

use std::path::PathBuf;
use std::path::Path;

use std::ffi::OsStr;
use std::ffi::OsString;

use std::io;

use crate::file_names;
use crate::image;

#[cfg(test)]
mod tests;


pub fn map_directory(source_path: &Path, destination_path: &Path) {
    //println!("Entered src '{:?}' and dst '{:?}'", source_path, destination_path);

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
    if let Some(extension) = source_file_path.extension() {
        if file_names::extension_is_image_extension(extension) {
            handle_source_image(source_file_path, destination_path);
        }
        // TODO: video
    }
    // Some files don't have extensions, so if one is missing, it's
    // not an image, so ignore it.
}

fn handle_source_image(source_image_path: &Path, destination_path: &Path) {
    let destination_image_name = file_names::destination_image_name_from_image_path(source_image_path);
    let destination_image_path = destination_path.join(destination_image_name);

    if !destination_image_path.exists() {
        image::open_compress_and_save_image(source_image_path, &destination_image_path);
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

        let destination_entry_path = &destination_entry.path();

        if destination_entry_path.is_dir() {
            handle_destination_dir(destination_entry_path, source_path);
        } else {
            handle_destination_file(destination_entry_path, source_path);
        }
    }
}

fn handle_destination_dir(destination_dir_path: &Path, source_path: &Path) {
    let destination_dir_name = destination_dir_path.file_name().expect("Could not get a file_name.");
    let corresponding_source_entry_path = source_path.join(destination_dir_name);

    if !corresponding_source_entry_path.is_dir() {
        fs::remove_dir_all(destination_dir_path).unwrap();
    }
    // No need to recursively call map_directory. If a destination dir
    // has a name that matches the source dir, then it will already have
    // been iterated in the source phase.
}

fn handle_destination_file(destination_file_path: &Path, source_path: &Path) {
    if let Some(extension) = destination_file_path.extension() {
        if file_names::extension_is_destination_file_extension(extension) {
            handle_destination_image(destination_file_path, source_path);
        } else {
            handle_destination_non_image_file(destination_file_path);
        }
    } else {
        handle_destination_extensionless_file(destination_file_path);
    }
}

fn handle_destination_image(destination_image_path: &Path, source_path: &Path) {
    let destination_image_name = destination_image_path.file_name().expect("Could not get a file name.").to_str().expect("Could not convert to str.");
    if let Some(corresponding_source_entry_name) = file_names::destination_image_name_to_source_image_name(destination_image_name) {
        let corresponding_source_entry_path = source_path.join(corresponding_source_entry_name);

        // The corresponding source entry must be a file, otherwise
        // it doesn't exist or is a dir.
        if !corresponding_source_entry_path.is_file() {
            fs::remove_file(destination_image_path).unwrap();
        }
    } else {
        // Some weird file_name that is very short. For sure invalid.
        fs::remove_file(destination_image_path).expect("Could not remove a file.");
    }    
}

fn handle_destination_non_image_file(destination_file_path: &Path) {
    fs::remove_file(destination_file_path).expect("Could not remove a file.");
}

fn handle_destination_extensionless_file(destination_file_path: &Path) {
    fs::remove_file(destination_file_path).expect("Could not remove a file.");
}
