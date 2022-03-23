use std::fs::File;
use std::io::Write;
use clap::{Command, load_yaml, Parser};
use std::path::Path;
use std::time::Instant;
use image::{GenericImageView, Pixel};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    input_path: String,

    #[clap(short, long)]
    output_path: String,

    #[clap(short, long)]
    resolution: Option<String>,

    #[clap(short, long)]
    big_charset: bool,

    #[clap(short, long)]
    time: bool,

    #[clap(short, long)]
    preview: bool,
}

fn main() {
    // Loading the yaml file containing all the information regarding the args
    let yaml = load_yaml!("cli.yaml");
    let _matches = Command::from_yaml(yaml).get_matches();

    let args = Args::parse();

    // If the file is valid, convert
    if check_file_validity(&args.input_path) {
        file_output(convert(preprocessing(&args.input_path, &args.resolution.unwrap_or("".to_string())), args.big_charset, args.time), &args.output_path);
        println!("\n\rConversion successful");

        if args.preview {
            preview(&args.input_path, args.big_charset);
        }
    }
}

fn convert(img: image::DynamicImage, big_charset: bool, time: bool) -> String {
    // Start of tracking conversion time
    let start = Instant::now();

    // String which will contain all the converted characters,
    // will be used for writing to output file
    let mut converted = String::new();

    // Vector which will contain the set of characters we will use for conversion
    let char_vec: Vec<char>;
    if big_charset == true {
        // Charset of 70
        char_vec = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ".chars().collect();
    } else {
        // Charset of 10
        char_vec = "@%#*+=-:. ".chars().collect();
    }

    let mut percentage: f32 = 0.0;

    let mut row = String::new();
    for y in 0..img.height() {
        for x in 0..img.width() {
            // Converting the RGB values of the pixel to just its luminosity
            let pixel = img.get_pixel(x, y).to_luma();
            let pixel_luma: f32 = pixel.0[0] as f32;
            // Converting the luminosity of the pixel to a proportional value
            // that can be fed to the Vector index
            let light: usize = (pixel_luma / 255.0 * ((char_vec.len() - 1) as f32)) as usize;
            // Adding the char that represents the pixel luminosity the best
            // to the current row of converted pixels
            row.push_str(char_vec[light].encode_utf8(&mut [0, 1]));
        }

        // Calculating the percentage of how much was already converted using the image height
        percentage = ((y+1) as f32 / img.height() as f32) * 100.0;

        /*
        Commenting this out because the progress bar takes wayyyyy too much time to print,
        slowing down the overall conversion time

        progress_bar(percentage as i32);
        */

        // Moving on to the next row of pixels
        converted.push_str(&[&row, "\n"].join(""));
        row.clear();
    }

    // End of tracking conversion time, calculating conversion time
    let elapsed = start.elapsed();

    if time {
        println!("Conversion completed in {}ms", elapsed.as_millis());
    }
    return converted;
}

fn file_output(text: String, output_path: &String) {
    // Creating the output file
    let mut file = File::create([output_path, "ascii_output.txt"].join("")).expect("Output file could not be generated");

    // Writing the converted image to the output file
    file.write_all(text.as_bytes()).expect("Unable to write to output file");
}

fn check_file_validity(path: &String) -> bool {
    // Checking if the given file path exists
    if Path::new(&path).exists() {
        println!("Filepath: {}", path);

        // Checking if the given file is of a supported file format
        if ["png", "jpg", "jpeg", "webp", "bmp"].contains(&Path::new(&path).extension().unwrap().to_str().unwrap().to_lowercase().as_str()) {
            return true;
        } else {
            panic!("Incorrect filetype!");
        }

    } else {
        panic!("Invalid Filepath!");
    }
}

/*
Commenting this out since it is not currently in use but might be useful again later on

fn progress_bar(percentage: i32) {
    for _ in 1..percentage {
        print!("\r[");
        for _ in 0..((percentage as f32/2.5) as i32) {
            print!("=");
        }
        for _ in ((percentage as f32/2.5) as i32)..40 {
            print!(" ");
        }
        print!("]");

        print!(" - {}% done", percentage);

        io::stdout().flush().unwrap();
    }
}
*/

fn preprocessing(input_path: &String, res: &String) -> image::DynamicImage {
    let mut img = image::open(input_path).unwrap();

    // If a resolution is given as an argument, convert the image to that resolution
    if res != "" {
        // Creating a Vector which contains the desired image resolution as x and y values
        let dim_vec: Vec<&str> = res.split('x').collect();
        // Resizing the image (very very ugly solution)
        img = img.resize(dim_vec[0].parse::<u32>().unwrap(), dim_vec[1].parse::<u32>().unwrap(), image::imageops::FilterType::Nearest);
    }

    return img;
}

fn preview(input_path: &String, big_charset: bool) {
    // Get the dimensions of the terminal window
    if let Some((w, h)) = term_size::dimensions() {
        // Print a converted ASCII-Image the size of the terminal window as a preview
        print!("{}", convert(preprocessing(input_path, &[w.to_string(), h.to_string()].join("x")), big_charset, false));
    } else {
        println!("Unable to get terminal size!");
    }
}
