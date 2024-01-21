use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};


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
    buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    x: Vec<u32>,
    y: Vec<u32>,
    err: [i32; 3],
) {
    let mut i: usize = 0;
    while i < x.len() {
        if i == 3 && x[i] == 0 {
            i += 1;
            continue;
        }

        let mut pixel = match buffer.get_pixel_checked(x[i], y[i]) {
            Some(pixel) => *pixel,
            None => {
                i += 1;
                continue;
            }
        };

        let mut color_index = 0;

        while color_index < 3 {
            let weighted_err = calculate_err(err[color_index] as f32, i);
            let adjusted_color_value = into_u8(weighted_err + pixel[color_index] as i16);
            pixel[color_index] = adjusted_color_value;

            color_index += 1;
        }

        buffer.put_pixel(x[i], y[i], pixel);

        i += 1;
    }
}

// Used to prevent an integer underflow from occurring.
fn subtract_absolute(a: u8, b: u8) -> u8 {
    if b > a {
        return b - a;
    }
    a - b
}

fn calculate_euclidean_distance(x_distance: i32, y_distance: i32, z_distance: i32) -> f32 {

    let difference_to_sqrt = (
        x_distance.pow(2) +
        y_distance.pow(2) +
        z_distance.pow(2)
    ) as f32;

    difference_to_sqrt.sqrt()
}

// Uses Euclidean distance, approximately
fn calculate_difference(start: &Rgb<u8>, end: &Rgb<u8>) -> u32 {

    let red_difference  : i32 = subtract_absolute(start[0], end[0]).into();
    let green_difference: i32 = subtract_absolute(start[1], end[1]).into();
    let blue_difference : i32 = subtract_absolute(start[2], end[2]).into();

    calculate_euclidean_distance(
        red_difference, 
        green_difference, 
        blue_difference
    ) as u32
}

fn calculate_error(old_pixel: Rgb<u8>, new_pixel: Rgb<u8>) -> [i32; 3] {

    let red_error  : i32 = old_pixel[0] as i32 - new_pixel[0] as i32;
    let green_error: i32 = old_pixel[1] as i32 - new_pixel[1] as i32;
    let blue_error : i32 = old_pixel[2] as i32 - new_pixel[2] as i32;

    [red_error, green_error, blue_error]
}


fn find_nearest_palette_color(pixel_color: &Rgb<u8>, palette: &Vec<Rgb<u8>>) -> Rgb<u8> {

    let mut palette_color: &Rgb<u8> = &Rgb([0,0,0]);
    let mut smallest_difference = 1000;

    for color in palette {
        let distance = calculate_difference(&pixel_color, color);
        if distance < smallest_difference {
            smallest_difference = distance;
            palette_color = color;
        }
    }

    *palette_color
}

// Shades are spread evenly
fn create_palette(num_of_colors: usize) -> Vec<Rgb<u8>> {

    let color_step_size: usize = 255 / (num_of_colors - 1);
    let black: Rgb<u8> = Rgb([0, 0, 0]);
    let mut palette = vec![black];

    let mut i: usize = 1;
    while i < num_of_colors {
        let shade = (color_step_size * i) as u8;
        palette.push(Rgb([shade, shade, shade]));

        i += 1;
    }
    
    palette
}


pub fn floyd_steinberg(img: &DynamicImage, num_of_colors: usize) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut buffer = img.to_rgb8();
    let available_colors = create_palette(num_of_colors);
    println!("{:?}", available_colors);

    // Iterate over the pixels
    for (imgx, imgy, _) in img.pixels() {
        let old_pixel = *(buffer.get_pixel(imgx, imgy));
        let new_pixel = find_nearest_palette_color(&old_pixel, &available_colors);

        buffer.put_pixel(imgx, imgy, new_pixel);
        let quant_error = calculate_error(old_pixel, new_pixel);
        
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
