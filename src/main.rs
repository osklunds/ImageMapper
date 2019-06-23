
#![allow(dead_code, unused_variables, unused_imports)]

use image_mapper;

use std::path::PathBuf;
//use std::ffi::OsString;


fn main() {
    
    let src = PathBuf::from(r"from");
    let dst = PathBuf::from(r"to");

    image_mapper::map_directory(&src, &dst);
    
    
}