
use image_mapper;

use std::path::PathBuf;
use std::ffi::OsString;


fn main() {
    let string: OsString = OsString::from("JPG");
    let r = image_mapper::is_image_extension(string.as_os_str());

    println!("{}", r);



    //let path = PathBuf::from(r"mapp");

    //image_mapper::map_directory(path.as_path(), path.as_path());
}