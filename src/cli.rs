pub struct Config {
    pub file_name: String,      // the file path of the image to convert
    pub output: String,         // the file path of the image created
    pub shades: usize,            // int ranging from 0 to 255, adjusts the amount of shades in the palatte. Default is 2.
    //replace: bool,              // a bool storing whether or not the image replaces the old image.
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
        let mut shades: usize = 2;

        if args_len >= 4 {
            match args[3].parse() {
                // values lower than 2 cause crashes, 2 is already set as default
                Ok(value) => {
                    if value > 1 {
                        shades = value
                    } else {
                        println!("Warning: number of shades cannot be lower than 2");
                        println!("Continuing with default: 2");
                    }
                }, 
                Err(_) => {
                    println!("Warning: cannot parse the number of shades");
                    println!("Continuing with default: 2");
                }
            };
        }
        
        Ok(Config { file_name, output, shades })
    }
}