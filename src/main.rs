mod dithering;
mod cli;
use cli::Config;
use std::process;
use std::error::Error;
use image::Rgb;

fn main() {

    let args: Vec<String> = std::env::args().collect();
    let config = cli::Config::new(&args).unwrap_or_else(|err | {
        println!("Config error: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(&config) {
        println!("Application error: {}", e);
        process::exit(1);
    }

    println!("File saved at: {}", config.output)
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {

    let img = image::open(&config.file_name)?;

    let palette = if config.color {
        create_color_palette(config.shades)
    } else {
        create_grey_palette(config.shades)
    };
    
    let buffer = dithering::floyd_steinberg(&img, palette);

    buffer.save(&config.output)?;

    Ok(())
}

fn create_grey_palette(num_of_colors: u8) -> Vec<Rgb<u8>> {
    
    let color_step_size: u8 = 255 / (num_of_colors -1);
    let mut palette = vec![Rgb([0, 0, 0])];

    let mut i: u8 = 1;
    while i < num_of_colors {
        let shade = color_step_size * i;
        palette.push(Rgb([shade, shade, shade]));

        i += 1;
    }

    palette
}

fn create_color_palette(num_of_colors: u8) -> Vec<Rgb<u8>> {

    let color_step_size: u8 = 255 / (num_of_colors - 1);
    let black: u8 = 0;

    let mut temporary_palette = vec![black];

    let mut i: u8 = 1;
    while i < num_of_colors {
        let shade = color_step_size * i;
        temporary_palette.push(shade);

        i += 1;
    }
    
    let mut palette = vec![];

    for red in &temporary_palette {

        for green in &temporary_palette {

            for blue in &temporary_palette {
                palette.push(Rgb([*red, *green, *blue]));
            }
        }
    }
    
    palette
}