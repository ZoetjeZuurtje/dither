use std::process;

pub struct Config {
    pub file_name: String,      // the file path of the image to convert
    pub output: String,         // the file path of the image created
    pub shades: u8,             // int ranging from 0 to 255, adjusts the amount of shades in the palette. Default is 2.
    pub color: bool,            // whether or not the output should be in color. `false` will produce black-and-white pictures.
}


// command should be used like so:
// dither input output [settings]
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        
        let args_len = args.len();

        if args_len < 2 {
            return Err("not enough arguments supplied.");
        }
    
        let file_name = args[1].clone();
        let output = if args_len >=3 { args[2].clone() } else { args[1].clone() };
        let mut shades = 2;
        let mut color = true;

        for arg in args {
            if arg == "--nocolor" {
                color = false;
                continue;
            };
            if arg == "--help" {
                println!( "Usage:\n\tdither [FILEPATH_INPUT] [FILEPATH_OUTPUT] [OPTION...]\n\n\tOptions are separated by spaces.\n\nOptions:\n\t--help\t\t\tShow this menu\n\t--nocolor\t\tOnly use greys for the palette\n\t[number]\t\tAmount of shades to use for Red, Green, and Blue color channels");
                process::exit(0);
            };
            if let Ok(value) = arg.parse() {
                if value > 2 {
                    shades = value
                } else {
                    println!("Warning: number of shades must be between 2 and 256 (exclusive)");
                    println!("Continuing with default: 2");
                }
            }
        }
        
        Ok(Config { file_name, output, shades, color })
    }
}