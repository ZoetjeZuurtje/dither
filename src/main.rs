use std::env;
use image::{Luma, GenericImageView, ImageBuffer};


fn main() {
    // CLI stuff
    let args: Vec<String> = env::args().collect();
    let img = image::open(&args[1]).unwrap();
    let options = &args[1];
    let mut name = "out.png";

    if options.contains('o') {
        name = &args[2];
    }

    let mut buffer: ImageBuffer<Luma<u8>, Vec<u8>> = img.to_luma8();
    let mut old_pixel: u8;
    let mut new_pixel: u8;
    let mut quant_error: f32;
    let mut result: f32;

    // Gets the width and height to prevent crashes due to out of bounds pixels
    let (width, mut height) = img.dimensions();
    height -= 1;

    // Iterate over the pixels
    for (imgx, imgy, _) in img.pixels() {

        // Floyd-Steinberg dithering!
        old_pixel = buffer.get_pixel(imgx, imgy)[0];
        new_pixel = 0;
        
        if old_pixel > 127 { new_pixel = 255 };

        buffer.put_pixel(imgx, imgy, Luma([new_pixel]));

        quant_error = old_pixel as f32 - new_pixel as f32;
        
        // Error diffusion
        if imgx + 1 < width {

            result = buffer.get_pixel(imgx + 1, imgy)[0] as f32 + (quant_error * 0.4375);
            if result > 255.0 { result = 255.0 };
            buffer.put_pixel(imgx + 1, imgy, Luma([result as u8])); // 7

            if imgy < height {
                result = buffer.get_pixel(imgx + 1, imgy + 1 )[0] as f32 + (quant_error * 0.0625);
                if result > 255.0 { result = 255.0 };
                buffer.put_pixel(imgx + 1, imgy + 1, Luma([result as u8])); // 1
            }
        };
    
        if imgx != 0 && imgy < height {
            result = buffer.get_pixel(imgx - 1, imgy + 1 )[0] as f32 + (quant_error * 0.3125);
            if result > 255.0 { result = 255.0 };
            buffer.put_pixel(imgx - 1, imgy + 1, Luma([result as u8])); // 5
        };
    
        if imgy < height {
            result = buffer.get_pixel(imgx, imgy + 1 )[0] as f32 + (quant_error * 0.1875);
            if result > 255.0 { result = 255.0 };
            buffer.put_pixel(imgx, imgy + 1, Luma([result as u8])); // 3
        };

    };

    buffer.save(name).unwrap();
}