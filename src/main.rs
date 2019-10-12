
#![allow(dead_code, unused_variables, unused_imports)]

use image_mapper;

use std::path::PathBuf;
//use std::ffi::OsString;


fn main() {
    
    let src = PathBuf::from("testing/from");
    let dst = PathBuf::from("testing/to");

    image_mapper::map_directory(&src, &dst);
    
    
}