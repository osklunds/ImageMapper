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
            println!("{:?}", "hej");
        }
        Err(MapperError::DstDoesNotExist) => {
            println!("{:?}", "hej");
        }
        Err(MapperError::SrcInsideDst) => {
            println!("{:?}", "hej");
        }
        Err(MapperError::DstInsideSrc) => {
            println!("{:?}", "hej");
        }
        Err(MapperError::DstTopLevelItemNotInSrc) => {
            println!("{:?}", "hej");
        }
    }
}
