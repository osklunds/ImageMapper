
#![allow(dead_code, unused_variables, unused_imports)]

use std::path::PathBuf;

mod file_names;
mod image;
mod mapper;

fn main() {    
    let src = PathBuf::from("testing/from");
    let dst = PathBuf::from("testing/to");

    mapper::map_directory(&src, &dst);
}
