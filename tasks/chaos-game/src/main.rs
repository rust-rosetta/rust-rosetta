extern crate image;
extern crate rand;

use std::f32::consts::PI;

use rand::prelude::*;

fn main() {
    let max_iterations = 50_000;
    let img_side = 800;
    let tri_size = 400.0;

    // Create a new ImgBuf
    let mut imgbuf = image::ImageBuffer::new(img_side, img_side);

    // Create triangle vertices
    let mut vertices = [(0.0, 0.0); 3];
    for (i, (x, y)) in vertices.iter_mut().enumerate() {
        *x = (img_side as f32 / 2.) + (tri_size / 2.) * (PI * i as f32 * 2. / 3.).cos();
        *y = (img_side as f32 / 2.) + (tri_size / 2.) * (PI * i as f32 * 2. / 3.).sin();
    }
    for &(x, y) in &vertices {
        imgbuf.put_pixel(x as u32, y as u32, image::Luma([255]));
    }

    // Iterate chaos game
    let mut rng = SmallRng::from_entropy();
    let mut x = img_side as f32 / 2.0;
    let mut y = img_side as f32 / 2.0;
    for _ in 0..max_iterations {
        let (choice_x, choice_y) = rng.choose(&vertices).unwrap();
        x = (x + choice_x) / 2.0;
        y = (y + choice_y) / 2.0;

        imgbuf.put_pixel(x as u32, y as u32, image::Luma([255]));
    }

    image::ImageLuma8(imgbuf).save("fractal.png").unwrap();
}
