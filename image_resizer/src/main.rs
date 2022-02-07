extern crate image;

use std::env;
use std::path::Path;
use std::fs::create_dir;

/// Main function - currently takes a single filepath
fn main() {
    let mut args = env::args().skip(1);
    assert_eq!(args.len(), 2, "Too many arguments!");
    let file_location = args.next().unwrap();
    let output_folder = args.next().unwrap();
    let dir_exists: bool = Path::new(&output_folder).is_dir();
    if !dir_exists {
        create_dir(&output_folder).expect("Couldn't create directory");
        println!("Created directory");
    }
    create_smaller_versions(&file_location, &output_folder);
}

/// Spawns three different sizes of png used as input in target folder
    ///
    /// # Arguments
    ///
    /// * `file` - A string slice that holds the file path
    /// * `output` - A string representing the target output folder
fn create_smaller_versions(file: &str, output: &str) {
    let file_base = Path::new(&file).file_stem().unwrap().to_str().unwrap();
    let one_x = "_1x.png";
    let _two_x = "_2x.png";
    let _three_x = "_3x.png";

    let img = image::open(&file).unwrap();
    let dims = image::image_dimensions(&file).unwrap();
    let width = dims.0;
    let height = dims.1;

    // Create 1x
    let out_one = img.resize(width, height/4, image::imageops::FilterType::Lanczos3);
    let one_file = format!("{}/{}{}", output, file_base, one_x);
    out_one.save(one_file).expect("Saving x1 image failed");

    // Create 2x
    let out_two = img.resize(width/2, height/2, image::imageops::FilterType::Lanczos3);
    let two_file = format!("{}/{}{}", output, file_base, _two_x);
    out_two.save(two_file).expect("Saving x2 image failed");

    // Create 3x
    let three_file = format!("{}/{}{}", output, file_base, _three_x);
    img.save(three_file).expect("Saving x3 image failed")
}

// TODO: Switch main to taking a input folder rather than single file
// TODO: Make it concurrent via crossbeam