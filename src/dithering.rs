use image::{Luma, DynamicImage, GenericImageView, ImageBuffer};


fn error_diffusion(buffer: &mut ImageBuffer<Luma<u8>, Vec<u8>>, x: u32, y: u32, err: f32) {

    let pixel = match buffer.get_pixel_checked(x, y) {
        Some(pixel) => pixel[0],
        None => return,
    };
    if err + pixel as f32 > 255.0 {
        buffer.put_pixel(x, y, Luma([255]));
        return;
    }
    let pixel: Luma<u8> = Luma([pixel + err as u8]);
    buffer.put_pixel(x, y, pixel);
}

// Floyd-Steinberg dithering!
pub fn floyd_steinberg(img: &DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut buffer: ImageBuffer<Luma<u8>, Vec<u8>> = img.to_luma8();
    let mut old_pixel: u8;
    let mut new_pixel: u8;
    let mut quant_error: f32;

    // Iterate over the pixels
    for (imgx, imgy, _) in img.pixels() {

        old_pixel = buffer.get_pixel(imgx, imgy)[0];
        new_pixel = 0;
        if old_pixel > 127 { new_pixel = 255 };

        buffer.put_pixel(imgx, imgy, Luma([new_pixel]));

        quant_error = old_pixel as f32 - new_pixel as f32;
        if quant_error < 0.0 { quant_error = 0.0; }
        // Error diffusion
        error_diffusion(&mut buffer, imgx + 1, imgy, quant_error * 0.4375);
        if imgx != 0 {
            error_diffusion(&mut buffer, imgx - 1, imgy + 1, quant_error * 0.1875);
        }
        error_diffusion(&mut buffer, imgx, imgy + 1, quant_error * 0.3125);
        error_diffusion(&mut buffer, imgx + 1, imgy + 1, quant_error * 0.0625);

    };

    buffer
}