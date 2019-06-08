
use image_mapper;

fn main() {
    let r = image_mapper::date_time_string_from_image_path("tests/test_image.jpg");

    println!("{}", r.as_str());
}