mod dithering;
mod cli;
use cli::get_options;
use image::DynamicImage;


fn main() {

    let settings = get_options();
    println!("{:#?}", settings.file_path_in);
    println!("{:#?}", settings.file_path_out);

    let img: DynamicImage;

    match image::open(settings.file_path_in) {
        Result::Ok(image) => img = image,
        Result::Err(_) => {
            panic!("\nCould not open file.\nCheck if the file path is correct and if you have the appropriate priviliges.");
        }
    };
    
    let buffer = dithering::floyd_steinberg(&img);

    buffer.save(settings.file_path_out).unwrap();
}