
#![allow(dead_code, unused_variables, unused_imports)]

use image_mapper;

use std::path::PathBuf;
//use std::ffi::OsString;


fn main() {
    /*
    let src = PathBuf::from(r"from");
    let dst = PathBuf::from(r"to");

    image_mapper::map_directory(&src, &dst);
    */

    let name = "   2004-04-09 17;33;15 test_image.jpg.jpg";
    let x = image_mapper::destination_file_name_to_file_name(name);
    
    println!("{}", x);
}