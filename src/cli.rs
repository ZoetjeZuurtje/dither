pub struct Settings {
    pub file_path_in: String,       // the file path of the image to convert
    pub file_path_out: String,      // the file path of the image created
    //speed: int,               // int ranging from 0 (slowest) to 5 (fastest), adjusts the dither algorithm accordingly. Faster speeds produce worse results
    //replace: bool,            // a bool storing whether or not the image replaces the old image.
}

pub fn get_options() -> Settings {
    let args: Vec<String> = std::env::args().collect();

    let mut options: Settings = Settings {
        file_path_in: "".to_string(),
        file_path_out: "out.jpg".to_string(),
        //replace: false,
    };

    options.file_path_in = args[1].clone();

    if args[2].contains("r") {
        options.file_path_out = options.file_path_in.clone();
        //options.replace = true;
    }

    if args[2].contains("o") && !args[2].contains("r") {
        options.file_path_out = args[3].clone();
    }
    
    options
}