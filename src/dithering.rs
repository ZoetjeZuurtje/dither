use image::{DynamicImage, GenericImageView, ImageBuffer, Luma};

fn calculate_err(error_value: f32, weight: usize) -> i16 {
    // Approximations for:
    //   .     *   7/16
    // 3/16  5/16  1/16
    const ERR_DIFF: [f32; 4] = [0.4375, 0.1875, 0.3125, 0.0625];

    let weighted_error = error_value * ERR_DIFF[weight];

    return weighted_error.floor() as i16;
}

fn error_diffusion(
    buffer: &mut ImageBuffer<Luma<u8>, Vec<u8>>,
    x: Vec<u32>,
    y: Vec<u32>,
    err: f32,
) {
    let mut adjusted_pixel_value;
    let mut i: usize = 0;
    while i < x.len() {
        if i == 3 && x[i] == 0 {
            i += 1;
            continue;
        }

        let pixel = match buffer.get_pixel_checked(x[i], y[i]) {
            Some(pixel) => pixel[0],
            None => {
                i += 1;
                continue;
            }
        };

        let weighted_err = calculate_err(err, i);

        adjusted_pixel_value = weighted_err + pixel as i16;

        if adjusted_pixel_value > 255 {
            adjusted_pixel_value = 255;
        } else if adjusted_pixel_value < 0 {
            adjusted_pixel_value = 0;
        };

        buffer.put_pixel(x[i], y[i], Luma([adjusted_pixel_value as u8]));

        i += 1;
    }
}

// Floyd-Steinberg dithering!
pub fn floyd_steinberg(img: &DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut buffer: ImageBuffer<Luma<u8>, Vec<u8>> = img.to_luma8();
    let mut quant_error: f32;

    // Iterate over the pixels
    for (imgx, imgy, _) in img.pixels() {
        let old_pixel = buffer.get_pixel(imgx, imgy)[0];
        let new_pixel = if old_pixel > 127 { 255 } else { 0 };

        buffer.put_pixel(imgx, imgy, Luma([new_pixel]));
        quant_error = old_pixel as f32 - new_pixel as f32;

        // Error diffusion
        if imgx == 0 {

            let rel_x_coords = vec![imgx + 1, imgx, imgx + 1];
            let rel_y_coords = vec![imgx + 1, imgx, imgx + 1];

            error_diffusion(
                &mut buffer,
                rel_x_coords,
                rel_y_coords,
                quant_error,
            );
            continue;
        };

        let rel_x_coords = vec![imgx + 1, imgx - 1, imgx, imgx + 1];
        let rel_y_coords = vec![imgy, imgy + 1, imgy + 1, imgy + 1];
        error_diffusion(
            &mut buffer,
            rel_x_coords,
            rel_y_coords,
            quant_error,
        );
    }

    buffer
}
