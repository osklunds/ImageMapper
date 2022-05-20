//#![allow(dead_code, unused_variables, unused_imports)]

use crate::settings::Settings;

mod file_names;
mod image;
mod mapper;
mod settings;

fn main() {
    let matches = settings::get_matches();
    let settings = Settings::new_from_matches(&matches);
    let source_path = settings::source_path_from_matches(&matches);
    let destination_path = settings::destination_path_from_matches(&matches);

    mapper::map_directory(&source_path, &destination_path, &settings);
}
