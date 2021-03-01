extern crate image;

use image::ColorType;
use std::path::Path;

// Framebuffer dimensions
const WIDTH: usize = 640;
const HEIGHT: usize = 480;

/// Formula for plasma at any particular address
fn plasma_pixel(x: f64, y: f64) -> f64 {
    ((x / 16.0).sin()
        + (y / 8.0).sin()
        + ((x + y) / 16.0).sin()
        + ((x * x + y * y).sqrt() / 8.0).sin()
        + 4.0)
        / 8.0
}

/// Precalculate plasma field lookup-table for performance
fn create_plasma_lut() -> Vec<f64> {
    let mut plasma: Vec<f64> = vec![0.0; WIDTH * HEIGHT];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            plasma[(y * WIDTH) + x] = plasma_pixel(x as f64, y as f64);
        }
    }
    plasma
}

/// Convert from HSV float(1.0,1.0,1.0) to RGB u8 tuple (255,255,255).
/// From https://crates.io/crates/palette 0.5.0 rgb.rs, simplified for example
fn hsv_to_rgb(hue: f64, saturation: f64, value: f64) -> (u8, u8, u8) {
    let c = value * saturation;
    let h = hue * 6.0;
    let x = c * (1.0 - (h % 2.0 - 1.0).abs());
    let m = value - c;
    let (red, green, blue) = match (h % 6.0).floor() as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    // Convert back to RGB (where components are integers from 0 to 255)
    (
        ((red + m) * 255.0).round() as u8,
        ((green + m) * 255.0).round() as u8,
        ((blue + m) * 255.0).round() as u8,
    )
}
fn main() {
    // The bitmap/framebuffer for our application. 3 u8 elements per output pixel
    let mut framebuffer: Vec<u8> = vec![0; WIDTH * HEIGHT * 3];
    // Generate a lookup table so we don't do too much math for every pixel.
    // Do it in a function so that the local one can be immutable.
    let plasma_lookup_table = create_plasma_lut();
    // For each (r,g,b) pixel in our output buffer
    for (index, rgb) in framebuffer.chunks_mut(3).enumerate() {
        // Lookup the precalculated plasma value
        let hue_lookup = plasma_lookup_table[index] % 1.0;
        let (red, green, blue) = hsv_to_rgb(hue_lookup, 1.0, 1.0);
        rgb[0] = red;
        rgb[1] = green;
        rgb[2] = blue;
    }
    // Save our plasma image to out.png
    let output_path = Path::new("out.png");
    match image::save_buffer(
        output_path,
        framebuffer.as_slice(),
        WIDTH as u32,
        HEIGHT as u32,
        ColorType::RGB(8),
    ) {
        Err(e) => println!("Error writing output image:\n{}", e),
        Ok(_) => println!("Output written to:\n{}", output_path.to_str().unwrap()),
    }
}
