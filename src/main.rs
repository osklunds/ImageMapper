
#![allow(dead_code, unused_variables, unused_imports)]

use image_mapper;

use std::path::PathBuf;
//use std::ffi::OsString;


fn main() {
    //let src = PathBuf::from(r"from");
    //let dst = PathBuf::from(r"to");

    //image_mapper::map_directory(&src, &dst);

    let hej = "2019-06-16 07:42:30   filename.jpg.jpg";
    let hej2 = image_mapper::destination_file_name_to_file_name(hej);

    println!("{}", hej2);
}