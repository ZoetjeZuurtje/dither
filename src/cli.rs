pub struct Settings {
    pub file_path_in: String,       // the file path of the image to convert
    pub file_path_out: String,      // the file path of the image created
    //speed: int,               // int ranging from 0 (slowest) to 5 (fastest), adjusts the dither algorithm accordingly. Faster speeds produce worse results
    //replace: bool,            // a bool storing whether or not the image replaces the old image.
}

pub fn get_options() -> Settings {
    let args: Vec<String> = std::env::args().collect();
    let args_len = args.len();

    let mut options: Settings = Settings {
        file_path_in: String::new(),
        file_path_out: String::new(),
    };

    if args_len < 2 {
        panic!("No arguments supplied. Please supply at least one argument.");
    }

    options.file_path_in = args[1].clone();
    options.file_path_out = format!("{}", args[1].clone());

    if args_len >= 4 {

        if args[2].contains("o") {
            options.file_path_out = args[3].clone();
        }
    }
    
    options
}