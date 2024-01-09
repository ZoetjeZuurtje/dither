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


fn absolute(num_1: &u8, num_2: &u8) -> u8 {

    if num_1 < num_2 {
        return num_2 - num_1;
    }

    num_1 - num_2
}

// Run a binary search to find the closest palate colour available
fn find_nearest_palate_colour(greyscale_color: u8, colours: &Vec<u8>) -> u8 {
    
    let mut smallest_difference = 255;
    let mut colour_to_use = 0;
    for colour in colours {

        let current_difference = absolute(&greyscale_color, colour);
        
        if current_difference < smallest_difference {
            smallest_difference = current_difference;
            colour_to_use = colour.clone();
        };

    }

    colour_to_use
}

// Floyd-Steinberg dithering!
pub fn floyd_steinberg(img: &DynamicImage, num_of_colours: usize) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut buffer: ImageBuffer<Luma<u8>, Vec<u8>> = img.to_luma8();
    let mut quant_error: f32;

    // Create the palate
    let mut available_colours = vec![0];
    let colour_step_size: usize = 255 / (num_of_colours - 1);
    let mut i: usize = 1;

    while i < num_of_colours {
        available_colours.push((colour_step_size * i) as u8);

        i += 1;
    }

    println!("{}", colour_step_size);
    println!("{:?}", available_colours);


    // Iterate over the pixels
    for (imgx, imgy, _) in img.pixels() {
        let old_pixel = buffer.get_pixel(imgx, imgy)[0];
        let new_pixel = find_nearest_palate_colour(old_pixel, &available_colours);

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
