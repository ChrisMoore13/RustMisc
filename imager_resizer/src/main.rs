extern crate image;

use std::env;
use std::path::Path;
// use image::GenericImageView;
// use std::path::Path;
// use std::fs::File;
// use image::GenericImageView;
// use image::{FilterType, GenericImage, Pixel};

fn main() {
    let mut args = env::args().skip(1);
    assert_eq!(args.len(), 3, "Not enough arguments!");
    let file_location = args.next().unwrap();
    let file_base = Path::new(&file_location).file_stem().unwrap().to_str().unwrap();
    let one_x = "_1x.png";
    let two_x = "_2x.png";
    let three_x = "_3x.png";

    let img = image::open(&file_location).unwrap();
    let dims = image::image_dimensions(&file_location).unwrap();
    let width = dims.0;
    let height = dims.1;

    // Create 1x
    let out_one = img.resize(width, height/4, image::imageops::FilterType::Lanczos3);
    let one_file = format!("{}{}", file_base, one_x);
    out_one.save(one_file).expect("Saving x1 image failed");

    // Create 2x
    let out_two = img.resize(width/2, height/2, image::imageops::FilterType::Lanczos3);
    let two_file = format!("{}{}", file_base, two_x);
    out_two.save(two_file).expect("Saving x2 image failed");

    // Create 3x
    let three_file = format!("{}{}", file_base, three_x);
    img.save(three_file).expect("Saving x3 image failed")
}
