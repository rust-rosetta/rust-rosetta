//! Contributed by Gavin Baker <gavinb@antonym.org>
//! Adapted from the Go version

use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::iter::repeat;

/// Simple 8-bit grayscale image
struct ImageGray8 {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

fn load_pgm(filename: &str) -> io::Result<ImageGray8> {
    // Open file
    let mut file = BufReader::new(File::open(filename)?);

    // Read header
    let mut magic_in = String::new();
    let _ = file.read_line(&mut magic_in)?;
    let mut width_in = String::new();
    let _ = file.read_line(&mut width_in)?;
    let mut height_in = String::new();
    let _ = file.read_line(&mut height_in)?;
    let mut maxval_in = String::new();
    let _ = file.read_line(&mut maxval_in)?;

    assert_eq!(magic_in, "P5\n");
    assert_eq!(maxval_in, "255\n");

    // Parse header
    let width = width_in
        .trim()
        .parse::<usize>()
        .map_err(|_| io::ErrorKind::InvalidData)?;
    let height: usize = height_in
        .trim()
        .parse::<usize>()
        .map_err(|_| io::ErrorKind::InvalidData)?;

    println!("Reading pgm file {}: {} x {}", filename, width, height);

    // Create image and allocate buffer
    let mut img = ImageGray8 {
        width,
        height,
        data: vec![],
    };

    // Read image data
    let expected_bytes = width * height;
    let bytes_read = file.read_to_end(&mut img.data)?;
    if bytes_read != expected_bytes {
        let kind = if bytes_read < expected_bytes {
            io::ErrorKind::UnexpectedEof
        } else {
            io::ErrorKind::InvalidData
        };
        let msg = format!("expected {} bytes", expected_bytes);
        return Err(io::Error::new(kind, msg));
    }

    Ok(img)
}

fn save_pgm(img: &ImageGray8, filename: &str) {
    // Open file
    let mut file = BufWriter::new(File::create(filename).unwrap());

    // Write header
    if let Err(e) = writeln!(&mut file, "P5\n{}\n{}\n255", img.width, img.height) {
        println!("Failed to write header: {}", e);
    }

    println!(
        "Writing pgm file {}: {} x {}",
        filename, img.width, img.height
    );

    // Write binary image data
    if let Err(e) = file.write_all(&(img.data[..])) {
        println!("Failed to image data: {}", e);
    }
}

#[allow(clippy::cast_precision_loss)]
#[allow(clippy::clippy::cast_possible_truncation)]
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

                let iry = out_height as i64 / 2 - (r / (dr as f64) + 0.5).floor() as i64;
                #[allow(clippy::clippy::cast_sign_loss)]
                let out_idx = (jtx as i64 + iry * out_width as i64) as usize;
                let col = accum.data[out_idx];
                if col > 0 {
                    accum.data[out_idx] = col - 1;
                }
            }
        }
    }
    accum
}

fn main() -> io::Result<()> {
    let image = load_pgm("resources/Pentagon.pgm")?;
    let accum = hough(&image, 460, 360);
    save_pgm(&accum, "hough.pgm");
    Ok(())
}
