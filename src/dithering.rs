use image::{DynamicImage, GenericImageView, ImageBuffer, Luma};

fn into_u8(number: i16) -> u8 {

    let unsigned_number: u8 = match number.try_into() {
        Ok(number) => number,
        Err(_) => {
            if number > 255 { 255 } else { 0 }
        },
    };

    unsigned_number
}

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

        let adjusted_pixel_value = into_u8(weighted_err + pixel as i16);

        buffer.put_pixel(x[i], y[i], Luma([adjusted_pixel_value as u8]));

        i += 1;
    }
}

// equivalent to `abs(a - b)`, but prevents an integer underflow from occurring
fn subtract_absolute(a: &u8, b: &u8) -> u8 {

    if a < b {
        return b - a;
    }

    a - b
}


fn find_nearest_palatte_color(greyscale_color: u8, colors: &Vec<u8>) -> u8 {
    
    let mut smallest_difference = 255;
    let mut color_to_use: &u8 = &0;
    for color in colors {

        let current_difference = subtract_absolute(&greyscale_color, color);
        
        if current_difference < smallest_difference {
            smallest_difference = current_difference;
            color_to_use = color;
        };

    }

    *color_to_use
}


// Shades are spread evenly
fn create_palatte_grey(shades: usize) -> Vec<u8> {

    let mut palatte = vec![0];
    let colour_step_size: usize = 255 / (shades - 1);
    let mut i: usize = 1;

    while i < shades {
        palatte.push((colour_step_size * i) as u8);

        i += 1;
    }

    palatte
}


pub fn floyd_steinberg(img: &DynamicImage, num_of_colors: usize) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut buffer: ImageBuffer<Luma<u8>, Vec<u8>> = img.to_luma8();

    let available_colors = create_palatte_grey(num_of_colors);

    // Iterate over the pixels
    for (imgx, imgy, _) in img.pixels() {
        let old_pixel = buffer.get_pixel(imgx, imgy)[0];
        let new_pixel = find_nearest_palatte_color(old_pixel, &available_colors);

        buffer.put_pixel(imgx, imgy, Luma([new_pixel]));
        let quant_error = old_pixel as f32 - new_pixel as f32;

        // Error diffusion
        // Ugly `if` statement is needed to prevent an integer underflow
        if imgx == 0 {

            let relative_x_coords = vec![imgx + 1, imgx, imgx + 1];
            let relative_y_coords = vec![imgx + 1, imgx, imgx + 1];

            error_diffusion(
                &mut buffer,
                relative_x_coords,
                relative_y_coords,
                quant_error,
            );
            continue;
        };

        let relative_x_coords = vec![imgx + 1, imgx - 1, imgx, imgx + 1];
        let relative_y_coords = vec![imgy, imgy + 1, imgy + 1, imgy + 1];
        error_diffusion(
            &mut buffer,
            relative_x_coords,
            relative_y_coords,
            quant_error,
        );
    }

    buffer
}
