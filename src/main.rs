mod dithering;
mod cli;
use cli::Config;
use std::process;
use std::error::Error;

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
    
    let buffer = dithering::floyd_steinberg(&img, config.shades);

    buffer.save(&config.output)?;

    Ok(())
}