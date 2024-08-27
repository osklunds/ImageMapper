use std::fs;
use std::fs::ReadDir;
use std::path::Path;
use std::result::Result;

use unwrap::unwrap;

use crate::file_names;
use crate::image;
use crate::settings::Settings;

#[cfg(test)]
mod tests;

pub fn map_directory(
    source_path: &Path,
    destination_path: &Path,
    settings: Settings,
) -> Result<(), MapperError> {
    map_directory_custom_opts(
        source_path,
        destination_path,
        settings,
        image::open_compress_and_save_image,
    )
}

// Custom options for test purposes
fn map_directory_custom_opts(
    source_path: &Path,
    destination_path: &Path,
    settings: Settings,
    open_compress_and_save_image: fn(&Path, &Path, &Settings) -> bool,
) -> Result<(), MapperError> {
    if !source_path.is_dir() {
        return Err(MapperError::SrcDoesNotExist);
    }
    if !destination_path.is_dir() {
        return Err(MapperError::DstDoesNotExist);
    }
    if is_path_subdir_of(&source_path, &destination_path) {
        return Err(MapperError::SrcInsideDst);
    }
    if is_path_subdir_of(&destination_path, &source_path) {
        return Err(MapperError::DstInsideSrc);
    }

    let opts = MapperOptions {
        settings,
        open_compress_and_save_image,
    };

    map_directory_int(source_path, destination_path, &opts);

    Ok(())
}

fn is_path_subdir_of(path_to_check: &Path, path_to_compare: &Path) -> bool {
    let path_to_check = fs::canonicalize(path_to_check).unwrap();
    let path_to_compare = fs::canonicalize(path_to_compare).unwrap();
    path_to_check.starts_with(&path_to_compare)
}

fn map_directory_int(
    source_path: &Path,
    destination_path: &Path,
    opts: &MapperOptions,
) {
    if opts.settings.verbose_print {
        println!(
            "Entered source: \"{}\" and destination: \"{}\"",
            source_path.display(),
            destination_path.display()
        );
    }

    ensure_path_is_directory(destination_path);

    iterate_source_entries(source_path, destination_path, opts);
    iterate_destination_entries(source_path, destination_path, opts);
}

fn ensure_path_is_directory(destination_path: &Path) {
    if destination_path.is_file() {
        unwrap!(
            fs::remove_file(destination_path),
            "Could not delete the directory \"{}\"",
            destination_path.display()
        );
    }
    if !destination_path.exists() {
        unwrap!(
            fs::create_dir(destination_path),
            "Could not create the directory \"{}\"",
            destination_path.display()
        );
    }
}

fn iterate_source_entries(
    source_path: &Path,
    destination_path: &Path,
    opts: &MapperOptions,
) {
    let source_entries = open_dir_to_iterator(source_path);

    for source_entry in source_entries {
        let source_entry =
            unwrap!(source_entry, "Could not open a source entry");

        let source_entry_path = &source_entry.path();

        if source_entry_path.is_dir() {
            handle_source_dir(source_entry_path, destination_path, opts);
        } else {
            handle_source_file(source_entry_path, destination_path, opts);
        }
    }
}

fn open_dir_to_iterator(path: &Path) -> ReadDir {
    unwrap!(
        fs::read_dir(path),
        "Could not open the directory \"{}\"",
        path.display()
    )
}

fn handle_source_dir(
    source_dir_path: &Path,
    destination_path: &Path,
    opts: &MapperOptions,
) {
    let source_dir_name = unwrap!(
        source_dir_path.file_name(),
        "Could not get the file name of a directory \"{}\"",
        source_dir_path.display()
    );
    let destination_dir_path = &destination_path.join(source_dir_name);

    map_directory_int(source_dir_path, destination_dir_path, opts);
}

fn handle_source_file(
    source_file_path: &Path,
    destination_path: &Path,
    opts: &MapperOptions,
) {
    if let Some(extension) = source_file_path.extension() {
        if file_names::extension_is_image_extension(extension) {
            handle_source_image(source_file_path, destination_path, opts);
        } else if file_names::extension_is_video_extension(extension)
            && opts.settings.include_videos
        {
            handle_source_video(source_file_path, destination_path, opts);
        }
    }
    // Some files dont have extensions, so if one is missing, its
    // not an image or a video, so ignore it.
}

fn handle_source_image(
    source_image_path: &Path,
    destination_path: &Path,
    opts: &MapperOptions,
) {
    let destination_image_name =
        file_names::destination_image_name_from_image_path(source_image_path);
    let destination_image_path = &destination_path.join(destination_image_name);

    if !destination_image_path.exists() {
        let successful = (opts.open_compress_and_save_image)(
            source_image_path,
            destination_image_path,
            &opts.settings,
        );

        if opts.settings.verbose_print && successful {
            println!("Created image \"{}\"", destination_image_path.display());
        }
    } else if opts.settings.verbose_print {
        println!(
            "Image \"{}\" aleady exists",
            destination_image_path.display()
        );
    }
}

fn handle_source_video(
    source_video_path: &Path,
    destination_path: &Path,
    opts: &MapperOptions,
) {
    let destination_video_name = unwrap!(
        source_video_path.file_name(),
        "Could not get the file name of a video \"{}\"",
        source_video_path.display()
    );
    let destination_video_path = &destination_path.join(destination_video_name);

    if !destination_video_path.exists() {
        unwrap!(
            fs::copy(source_video_path, destination_video_path),
            "Could not copy a video \"{}\" to \"{}\"",
            source_video_path.display(),
            destination_video_path.display()
        );

        if opts.settings.verbose_print {
            println!("Created video \"{}\"", destination_video_path.display());
        }
    } else if opts.settings.verbose_print {
        println!(
            "Video \"{}\" aleady exists",
            destination_video_path.display()
        );
    }
}

fn iterate_destination_entries(
    source_path: &Path,
    destination_path: &Path,
    opts: &MapperOptions,
) {
    let destination_entries = open_dir_to_iterator(destination_path);

    for destination_entry in destination_entries {
        let destination_entry =
            unwrap!(destination_entry, "Could not open a destination entry");

        let destination_entry_path = &destination_entry.path();

        if destination_entry_path.is_dir() {
            handle_destination_dir(
                destination_entry_path,
                source_path,
                opts,
            );
        } else {
            handle_destination_file(
                destination_entry_path,
                source_path,
                opts,
            );
        }
    }
}

fn handle_destination_dir(
    destination_dir_path: &Path,
    source_path: &Path,
    opts: &MapperOptions,
) {
    let destination_dir_name = unwrap!(
        destination_dir_path.file_name(),
        "Could not get the name of a directory \"{}\"",
        destination_dir_path.display()
    );
    let corresponding_source_entry_path =
        source_path.join(destination_dir_name);

    if !corresponding_source_entry_path.is_dir() {
        unwrap!(
            fs::remove_dir_all(destination_dir_path),
            "Could not remove a destination directory \"{}\"",
            destination_dir_path.display()
        );

        if opts.settings.verbose_print {
            println!("Deleted \"{}\"", destination_dir_path.display());
        }
    }
    // No need to recursively call map_directory_int. If a destination dir
    // has a name that matches the source dir, then it will already have
    // been iterated in the source phase.
}

fn handle_destination_file(
    destination_file_path: &Path,
    source_path: &Path,
    opts: &MapperOptions,
) {
    if let Some(extension) = destination_file_path.extension() {
        if file_names::extension_is_destination_image_extension(extension) {
            handle_destination_image(
                destination_file_path,
                source_path,
                opts,
            );
        } else if file_names::extension_is_video_extension(extension) {
            handle_destination_video(
                destination_file_path,
                source_path,
                opts,
            );
        } else {
            handle_destination_other_file(destination_file_path, opts);
        }
    } else {
        handle_destination_extensionless_file(destination_file_path, opts);
    }
}

fn handle_destination_image(
    destination_image_path: &Path,
    source_path: &Path,
    opts: &MapperOptions,
) {
    let destination_image_name = destination_image_path
        .file_name()
        .expect("Could not get a file name.")
        .to_str()
        .expect("Could not convert to str.");
    let mut deleted = false;

    if let Some(corresponding_source_entry_name) =
        file_names::destination_image_name_to_source_image_name(
            destination_image_name,
        )
    {
        let corresponding_source_entry_path =
            source_path.join(corresponding_source_entry_name);

        // The corresponding source entry must be a file, otherwise
        // it doesnt exist or is a dir.
        if !corresponding_source_entry_path.is_file() {
            fs::remove_file(destination_image_path).unwrap();
            deleted = true;
        }
    } else {
        // Some weird file_name that is very short. For sure invalid.
        fs::remove_file(destination_image_path)
            .expect("Could not remove a file.");
        deleted = true;
    }

    if opts.settings.verbose_print && deleted {
        println!("Deleted {}", destination_image_path.display());
    }
}

fn handle_destination_video(
    destination_video_path: &Path,
    source_path: &Path,
    opts: &MapperOptions,
) {
    let destination_video_name = unwrap!(
        destination_video_path.file_name(),
        "Could not get the file name from \"{}\"",
        destination_video_path.display()
    );
    let corresponding_source_entry_path =
        source_path.join(destination_video_name);

    // The corresponding source entry must be a file, otherwise
    // it doesnt exist or is a dir.
    // We must also want to have videos in the destination.
    if !(corresponding_source_entry_path.is_file()
        && opts.settings.include_videos)
    {
        unwrap!(
            fs::remove_file(destination_video_path),
            "Could not delete \"{}\"",
            destination_video_path.display()
        );

        if opts.settings.verbose_print {
            println!("Deleted \"{}\"", destination_video_path.display());
        }
    }
}

fn handle_destination_other_file(
    destination_file_path: &Path,
    opts: &MapperOptions,
) {
    unwrap!(
        fs::remove_file(destination_file_path),
        "Could not delete \"{}\"",
        destination_file_path.display()
    );
    if opts.settings.verbose_print {
        println!("Deleted \"{}\"", destination_file_path.display());
    }
}

fn handle_destination_extensionless_file(
    destination_file_path: &Path,
    opts: &MapperOptions,
) {
    unwrap!(
        fs::remove_file(destination_file_path),
        "Could not delete \"{}\"",
        destination_file_path.display()
    );
    if opts.settings.verbose_print {
        println!("Deleted \"{}\"", destination_file_path.display());
    }
}

struct MapperOptions {
    settings: Settings,
    open_compress_and_save_image: fn(&Path, &Path, &Settings) -> bool,
}

#[derive(Debug, PartialEq)]
pub enum MapperError {
    SrcDoesNotExist,
    DstDoesNotExist,
    SrcInsideDst,
    DstInsideSrc,
    // DstTopLevelDirNotInSrc,
}
