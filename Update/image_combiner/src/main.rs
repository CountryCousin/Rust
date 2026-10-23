
mod args;
use args::Args;
use image::{DynamicImage, ImageFormat, ImageReader, io::Reader}; 
use std::io::BufReader;
use std::fs::File;

#[derive(Debug)]
enum ImageDataErrors {
  DifferentImageFormats,
}

fn main()-> Result<(), ImageDataErrors> {
    let args = Args::new();
    println!("args: {:?}", args);
    let (image_1, image__1_format) = find_image_from_path(args.image_1);
    let (image_2, image_2_format) = find_image_from_path(args.image_2);

    if image__1_format != image_2_format {
        return Err(ImageDataErrors::DifferentImageFormats)
    }
    Ok(())
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader : ImageReader<BufReader<File>> = ImageReader::open(path).unwrap();
    // let image_reader : Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format :ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();
    (image, image_format)
}

// Resize the Images to Match
fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: ( u32, u32)) -> (u32, u32) {
    let pix_1 = dim_1.0 * dim_1.1;
    let pix_2 = dim_2.0 * dim_2.1;
    return if pix_1 < pix_2 {dim_1} else {dim_2}
}