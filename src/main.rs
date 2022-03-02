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
    big_charset: bool,

    #[clap(short, long)]
    time: bool,
}

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let _matches = Command::from_yaml(yaml).get_matches();

    let args = Args::parse();

    if check_file_validity(&args.input_path) {
        file_output(convert(&args.input_path, args.big_charset, args.time), &args.output_path);
    }
}

fn convert(input_path: &String, big_charset: bool, time: bool) -> String {
    let start = Instant::now();

    let img = image::open(input_path).unwrap();
    let mut converted = String::new();

    let char_vec: Vec<char>;
    if big_charset == true {
        char_vec = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ".chars().collect();
    } else {
        char_vec = "@%#*+=-:. ".chars().collect();
    }

    let mut row = String::new();
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y).to_luma();
            let pixel_luma: f32 = pixel.0[0] as f32;
            let light: usize = (pixel_luma / 255.0 * ((char_vec.len() - 1) as f32)) as usize;
            row.push_str(char_vec[light].encode_utf8(&mut [0, 1]));
        }

        converted.push_str(&[&row, "\n"].join(""));
        row.clear();
    }

    let elapsed = start.elapsed();

    println!("Conversion successful");
    if time {
        println!("Conversion completed in {}ms", elapsed.as_millis());
    }
    return converted;
}

fn file_output(text: String, output_path: &String) {
    let mut file = File::create([output_path, "ascii_output.txt"].join("")).expect("Output file could not be generated");
    file.write_all(text.as_bytes()).expect("Unable to write to output file");
}

fn check_file_validity(path: &String) -> bool {
    if Path::new(&path).exists() {
        println!("Filepath: {}", path);

        if ["png", "jpg", "jpeg"].contains(&Path::new(&path).extension().unwrap().to_str().unwrap()) {
            return true;
        } else {
            panic!("Incorrect filetype!");
        }

    } else {
        panic!("Invalid Filepath!");
    }
}
