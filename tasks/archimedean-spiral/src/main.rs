
#[macro_use(px)]
extern crate bmp;

use bmp::{Image, Pixel};
use std::f64;

fn main() {
    let width = 600u32;
    let half_width = (width / 2) as i32;
    let mut img = Image::new(width, width);
    let draw_color = px!(255, 128, 128);

    // Constants defining the spiral size.
    let a = 1.0_f64;
    let b = 9.0_f64;

    // max_angle = number of spirals * 2pi.
    let max_angle = 5.0_f64 * 2.0_f64 * f64::consts::PI;

    let mut theta = 0.0_f64;
    while theta < max_angle {
        theta = theta + 0.002_f64;

        let r = a + b * theta;
        let x = (r * theta.cos()) as i32 + half_width;
        let y = (r * theta.sin()) as i32 + half_width;
        img.set_pixel(x as u32, y as u32, draw_color);
    }

    // Save the image
    let _ = img.save("archimedean_spiral.bmp").unwrap_or_else(|e| panic!("Failed to save: {}", e));
}
