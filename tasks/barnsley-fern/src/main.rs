extern crate rand;
extern crate raster;

use rand::Rng;

const MAX_ITER: u32 = 200_000;
const HEIGHT: i32 = 640;
const WIDTH: i32 = 640;
const OUTPUT: &str = "fractal.png";

fn transform(x: f64, y: f64) -> (f64, f64) {
    let cx: f64;
    let cy: f64;

    match rand::thread_rng().gen::<f32>() {
        r if r <= 0.01 => {
            cx = 0.0;
            cy = 0.16 * y;
        }
        r if r <= 0.08 => {
            cx = 0.2 * x - 0.26 * y;
            cy = 0.23 * x + 0.22 * y + 1.6;
        }
        r if r <= 0.15 => {
            cx = -0.15 * x + 0.28 * y;
            cy = 0.26 * x + 0.26 * y + 0.44;
        }
        _ => {
            cx = 0.85 * x + 0.04 * y;
            cy = -0.04 * x + 0.85 * y + 1.6;
        }
    }

    (cx, cy)
}

fn main() {
    let mut image = raster::Image::blank(WIDTH, HEIGHT);

    raster::editor::fill(&mut image, raster::Color::white()).unwrap();

    let mut x = 0.0;
    let mut y = 0.0;

    for _ in 0..MAX_ITER {
        let (cx, cy) = transform(x, y);

        let target_x = (WIDTH as f64 / 2.0 + cx * (WIDTH as f64) / 11.0).round() as i32;
        let target_y = (HEIGHT as f64 - cy * (HEIGHT as f64) / 11.0).round() as i32;
        let color = raster::Color::rgb(50, 205, 50);

        image.set_pixel(target_x, target_y, color).unwrap();

        x = cx;
        y = cy;
    }

    raster::save(&image, OUTPUT).unwrap();
}
