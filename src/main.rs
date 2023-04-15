mod dithering;
use std::env;

use image::DynamicImage;


fn main() {
    // CLI stuff
    let args: Vec<String> = env::args().collect();
    let img: DynamicImage;
    let options = &args[2];
    let mut name = "out.png";

    match image::open(&args[1]) {
        Result::Ok(image) => img = image,
        Result::Err(error) => panic!("Error: {}", error),
    };
    if options.contains('o') {
        name = &args[3];
    };
    
    let buffer = dithering::floyd_steinberg(&img);

    buffer.save(name).unwrap();
}