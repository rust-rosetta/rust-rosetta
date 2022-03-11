extern crate rand;
extern crate raster;

use rand::Rng;

fn main() {
    let max_iterations = 200_000u32;
    let height = 640i32;
    let width = 640i32;

    let mut rng = rand::thread_rng();
    let mut image = raster::Image::blank(width, height);
    raster::editor::fill(&mut image, raster::Color::white()).unwrap();

    let mut x = 0.;
    let mut y = 0.;
    for _ in 0..max_iterations {
        let r = rng.gen::<f32>();
        let cx: f64;
        let cy: f64;

        if r <= 0.01 {
            cx = 0f64;
            cy = 0.16 * y as f64;
        } else if r <= 0.08 {
            cx = 0.2 * x as f64 - 0.26 * y as f64;
            cy = 0.23 * x as f64 + 0.22 * y as f64 + 1.6;
        } else if r <= 0.15 {
            cx = -0.15 * x as f64 + 0.28 * y as f64;
            cy = 0.26 * x as f64 + 0.26 * y as f64 + 0.44;
        } else {
            cx = 0.85 * x as f64 + 0.04 * y as f64;
            cy = -0.04 * x as f64 + 0.85 * y as f64 + 1.6;
        }
        x = cx;
        y = cy;

        let _ = image.set_pixel(
            ((width as f64) / 2. + x * (width as f64) / 11.).round() as i32,
            ((height as f64) - y * (height as f64) / 11.).round() as i32,
            raster::Color::rgb(50, 205, 50),
        );
    }

    raster::save(&image, "fractal.png").unwrap();
}
