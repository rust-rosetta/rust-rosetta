//! Contributed by Gavin Baker <gavinb@antonym.org>
//! Adapted from the Go version

use std::io::{BufReader, BufRead, BufWriter, Write, Read};
use std::fs::File;
use std::iter::repeat;

/// Simple 8-bit grayscale image
struct ImageGray8 {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

fn load_pgm(filename: &str) -> ImageGray8 {
    // Open file
    let mut file = BufReader::new(File::open(filename).unwrap());

    // Read header
    let mut magic_in = String::new();
    let _ = file.read_line(&mut magic_in).unwrap();
    let mut width_in = String::new();
    let _ = file.read_line(&mut width_in).unwrap();
    let mut height_in = String::new();
    let _ = file.read_line(&mut height_in).unwrap();
    let mut maxval_in = String::new();
    let _ = file.read_line(&mut maxval_in).unwrap();

    assert_eq!(magic_in, "P5\n");
    assert_eq!(maxval_in, "255\n");

    // Parse header

    let width = width_in.trim().parse::<usize>().unwrap();
    let height: usize = height_in.trim().parse::<usize>().unwrap();

    println!("Reading pgm file {}: {} x {}", filename, width, height);

    // Create image and allocate buffer

    let mut img = ImageGray8 {
        width: width,
        height: height,
        data: repeat(0u8).take(width * height).collect(),
    };

    // Read image data
    let len = img.data.len();
    match file.read(&mut img.data) {
        Ok(bytes_read) if bytes_read == len => println!("Read {} bytes", bytes_read),
        Ok(bytes_read) => println!("Error: read {} bytes, expected {}", bytes_read, len),
        Err(e) => println!("error reading: {}", e),
    }

    img
}

fn save_pgm(img: &ImageGray8, filename: &str) {
    // Open file
    let mut file = BufWriter::new(File::create(filename).unwrap());

    // Write header
    if let Err(e) = writeln!(&mut file, "P5\n{}\n{}\n255", img.width, img.height) {
        println!("Failed to write header: {}", e);
    }

    println!("Writing pgm file {}: {} x {}",
             filename,
             img.width,
             img.height);

    // Write binary image data
    if let Err(e) = file.write_all(&(img.data[..])) {
        println!("Failed to image data: {}", e);
    }
}

fn hough(image: &ImageGray8, out_width: usize, out_height: usize) -> ImageGray8 {
    let in_width = image.width;
    let in_height = image.height;

    // Allocate accumulation buffer
    let out_height = ((out_height / 2) * 2) as usize;
    let mut accum = ImageGray8 {
        width: out_width,
        height: out_height,
        data: repeat(255).take(out_width * out_height).collect(),
    };

    // Transform extents
    let rmax = (in_width as f64).hypot(in_height as f64);
    let dr = rmax / (out_height / 2) as f64;
    let dth = std::f64::consts::PI / out_width as f64;

    // Process input image in raster order
    for y in 0..in_height {
        for x in 0..in_width {
            let in_idx = y * in_width + x;
            let col = image.data[in_idx];
            if col == 255 {
                continue;
            }

            // Project into rho,theta space
            for jtx in 0..out_width {
                let th = dth * (jtx as f64);
                let r = (x as f64) * (th.cos()) + (y as f64) * (th.sin());

                let iry = out_height / 2 - (r / (dr as f64) + 0.5).floor() as usize;
                let out_idx = jtx + iry * out_width;
                let col = accum.data[out_idx];
                if col > 0 {
                    accum.data[out_idx] = col - 1;
                }
            }
        }
    }
    accum
}

fn main() {
    let image = load_pgm("resources/Pentagon.pgm");

    let accum = hough(&image, 460, 360);

    save_pgm(&accum, "hough.pgm");
}
