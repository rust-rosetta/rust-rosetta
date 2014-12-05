// Implements http://rosettacode.org/wiki/Hough_transform
//
// Contributed by Gavin Baker <gavinb@antonym.org>
// Adapted from the Go version

#![allow(dead_code)]

use std::num::{Float, FloatMath};
use std::io::{BufferedReader, BufferedWriter, File};

// Simple 8-bit grayscale image

struct ImageGray8 {
    width: uint,
    height: uint,
    data: Vec<u8>,
}

fn load_pgm(filename: &str) -> ImageGray8 {

    // Open file

    let path = Path::new(filename);
    let mut file = BufferedReader::new(File::open(&path));

    // Read header

    let magic_in = file.read_line().unwrap();
    let width_in = file.read_line().unwrap();
    let height_in = file.read_line().unwrap();
    let maxval_in = file.read_line().unwrap();

    assert_eq!(magic_in, "P5\n");
    assert_eq!(maxval_in, "255\n");

    // Parse header

    let width: uint = from_str(width_in.trim()).unwrap();
    let height: uint = from_str(height_in.trim()).unwrap();

    println!("Reading pgm file {}: {} x {}", filename, width, height);

    // Create image and allocate buffer

    let mut img = ImageGray8 {
        width: width,
        height: height,
        data: Vec::from_elem(width*height, 0),
    };

    // Read image data

    match file.read_at_least(img.data.len(), img.data.as_mut_slice()) {
        Ok(bytes_read) => println!("Read {} bytes", bytes_read),
        Err(e) => println!("error reading: {}", e)
    }

    img
}

fn save_pgm(img: &ImageGray8, filename: &str) {

    // Open file

    let path = Path::new(filename);
    let mut file = BufferedWriter::new(File::create(&path));

    // Write header

    match file.write_line(format!("P5\n{}\n{}\n255", img.width, img.height).as_slice()) {
        Err(e) => println!("Failed to write header: {}", e),
        _ => {},
    }

    println!("Writing pgm file {}: {} x {}", filename, img.width, img.height);

    // Write binary image data

    match file.write(img.data.as_slice()) {
        Err(e) => println!("Failed to image data: {}", e),
        _ => {},
    }
}

fn hough(image: &ImageGray8, out_width: uint, out_height: uint) -> ImageGray8 {

    let in_width = image.width;
    let in_height = image.height;

    // Allocate accumulation buffer

    let out_height = ((out_height/2) * 2) as uint;
    let mut accum = ImageGray8 {
        width: out_width,
        height: out_height,
        data: Vec::from_elem(out_width*out_height, 255),
    };

    // Transform extents

    let rmax = (in_width as f64).hypot(in_height as f64);
    let dr = rmax / (out_height/2) as f64;
    let dth = std::f64::consts::PI / out_width as f64;

    // Process input image in raster order

    for y in range(0, in_height) {
        for x in range(0, in_width) {
            let in_idx = y*in_width+x;
            let col = image.data[in_idx];
            if col == 255 {
                continue;
            }

            // Project into rho,theta space

            for jtx in range(0, out_width) {
                let th = dth * (jtx as f64);
                let r = (x as f64)*(th.cos()) + (y as f64)*(th.sin());

                let iry = out_height/2 - (r/(dr as f64)+0.5).floor() as uint;
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

#[cfg(not(test))]
fn main() {

    let image = load_pgm("../src/resources/Pentagon.pgm");

    let accum = hough(&image, 460, 360);

    save_pgm(&accum, "hough.pgm");
}
