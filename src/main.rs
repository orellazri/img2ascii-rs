use clap::Clap;
use image::{imageops::FilterType, io::Reader as ImageReader, GenericImageView, Pixel};

#[derive(Clap)]
struct Args {
    file_name: String,
    shrink_factor: Option<u8>,
}

fn main() {
    // Parse arguments
    let args = Args::parse();
    let file_name = args.file_name;
    let shrink_factor = args.shrink_factor.unwrap_or(5) as u32;

    // Load image and resize
    let img = ImageReader::open(file_name).unwrap().decode().unwrap();
    let img = img.resize(img.width() / shrink_factor, img.height() / shrink_factor, FilterType::Nearest);
    let width = img.dimensions().0 as usize;
    let height = img.dimensions().0 as usize;

    // Prepare ASCII values
    let value_chars = "`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
    let value_chars_bytes = value_chars.as_bytes();
    let mut ascii_pixels: Vec<char> = Vec::with_capacity(width * height);

    // Map each pixel to an ASCII character
    for pixel in img.pixels() {
        let channels = pixel.2.channels();
        let r = channels[0] as u16;
        let g = channels[1] as u16;
        let b = channels[2] as u16;

        let value = ((r + g + b) / 3) as f32; // Average brightness value. Can also do lightness/luminosity

        let mut value_index: usize = ((value / 255.0) * value_chars.len() as f32).round() as usize;
        if value_index == value_chars.len() {
            value_index -= 1;
        }
        ascii_pixels.push(value_chars_bytes[value_index] as char);
    }

    // Print ASCII
    for (i, c) in ascii_pixels.iter().enumerate() {
        print!("{}{}{}", c, c, c);
        if i > 0 && i % width == 0 {
            println!();
        }
    }
}
