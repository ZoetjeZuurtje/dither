pub struct Config {
    pub file_name: String,      // the file path of the image to convert
    pub output: String,         // the file path of the image created
    pub shades: u8,            // int ranging from 0 to 255, adjusts the amount of shades in the palatte. Default is 2.
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
        let mut output = format!("{}", args[1].clone());
        let mut shades: u8 = 2;
    
        if args_len >= 3 {
            output = args[2].clone();
        }

        if args_len >= 4 {
            match args[3].parse() {
                Ok(value) => shades = value,
                Err(_) => return Err("could not parse the number of shades")
            };
        }
        
        Ok(Config { file_name, output, shades })
    }
}