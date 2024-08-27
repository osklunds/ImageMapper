//#![allow(dead_code, unused_variables, unused_imports)]

use crate::settings::Settings;

mod file_names;
mod image;
mod mapper;
mod settings;

use mapper::MapperError;

fn main() {
    let matches = settings::get_matches();
    let settings = Settings::new_from_matches(&matches);
    let source_path = settings::source_path_from_matches(&matches);
    let destination_path = settings::destination_path_from_matches(&matches);

    let result =
        mapper::map_directory(&source_path, &destination_path, settings);

    match result {
        Ok(_) => {}
        Err(MapperError::SrcDoesNotExist) => {
            println!("Error: The specified source directory '{}' does not exist or is not a directory", source_path.display());
        }
        Err(MapperError::DstDoesNotExist) => {
            println!("Error: The specified destination directory '{}' does not exist or is not a directory", destination_path.display());
        }
        Err(MapperError::SrcInsideDst) => {
            println!("Error: The specified source '{}' lies inside the specified destination directory '{}'", source_path.display(), destination_path.display());
        }
        Err(MapperError::DstInsideSrc) => {
            println!("Error: The specified destination '{}' lies inside the specified source directory '{}'", destination_path.display(), source_path.display());
        }
        Err(MapperError::DstTopLevelEntryNotInSrc(missing_path)) => {
            println!("Error: The entry '{}' exists as a top-level entry in the specified destination directory, but it does NOT exist in the specified source directory. Running the program like this would cause it to be deleted, so as a safety precaution, the program stops here, because this error might indicate that an incorrect destination directory was specified. Proceding might cause many files to be deleted by mistake. Double check the destination directory and delete the entry manually instead. Note, this check is only done for top-level entries, NOT for entries inside sub directories.", missing_path.display());
        }
    }
}
