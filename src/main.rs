
#![allow(dead_code, unused_variables, unused_imports)]

use std::path::PathBuf;

use crate::settings::Settings;

mod file_names;
mod image;
mod mapper;
mod settings;

fn main() {
    let settings = Settings::new_from_arguments();
    
    let src = PathBuf::from("testing/from");
    let dst = PathBuf::from("testing/to");

    mapper::map_directory(&src, &dst);
}
