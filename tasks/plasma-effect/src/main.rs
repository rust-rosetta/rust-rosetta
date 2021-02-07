// Dependencies: minifb = "0.16"
// This runs much faster in a Release build.
// You can also make it *much* faster again by including Rayon
// and replace iter_mut with par_iter_mut
extern crate minifb;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
// Limit to max ~30 fps update rate. Try to match what's achievable for best results
const FPS: u64 = 30;
// Speed that we'll transition through colours. 0.02 for 30fps, 0.01 for 60fps, etc
const HUESHIFT_SPEED: f64 = 0.6 / FPS as f64;

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

/// Convert from HSV float(1.0,1.0,1.0) to RGB u32 packed with u8 (255,255,255,255).
///
/// From https://crates.io/crates/palette 0.5.0 rgb.rs, simplified for example
fn hsv_to_rgb32(hue: f64, saturation: f64, value: f64) -> u32 {
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
    // Convert back to ARGB (where components are integers from 0 to 255)
    let (ri, gi, bi) = (
        ((red + m) * 255.0).round() as u32,
        ((green + m) * 255.0).round() as u32,
        ((blue + m) * 255.0).round() as u32,
    );
    // And then shift and bitwise or to pack them in a unsigned 32bit to return
    ri << 16 | gi << 8 | bi
}

fn main() {
    // Create our window so we've got somewhere to put our pixels
    let mut window = Window::new(
        "RosettaCode Plasma Effect! - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window.limit_update_rate(Some(std::time::Duration::from_micros(1_000_000 / FPS)));

    // Colour shift that we'll increment every frame to give moving plasma
    let mut hue_shift: f64 = 0.0;
    // The bitmap/framebuffer for our application.
    let mut framebuffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    // Generate a lookup table so we don't do too much math for every pixel.
    // Do it in a function so that the local one can be immutable.
    let plasma_lookup_table = create_plasma_lut();

    // Do this until the user closes the app
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Increment the color shift every frame so we change over time
        hue_shift = (hue_shift + HUESHIFT_SPEED) % 1.0;
        // For each element in our output buffer
        framebuffer
            .iter_mut()
            .enumerate()
            .for_each(|(index, buf_elem)| {
                // Lookup the precalculated plasma value
                let hue_lookup = plasma_lookup_table[index];
                // Add a colour offset so it shifts over time
                let hue_with_shift = (hue_shift + hue_lookup) % 1.0;
                // Convert that colour into an ARGB value, store in buffer
                *buf_elem = hsv_to_rgb32(hue_with_shift, 1.0, 1.0);
            });
        // Draw our buffer to the screen
        window
            .update_with_buffer(&framebuffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
