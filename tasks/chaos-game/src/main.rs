extern crate image;
extern crate rand;

use rand::Rng;
use std::f32;
use std::fs::File;

fn main() {
    let max_iterations = 50_000u32;
    let img_side = 800u32;
    let tri_size = 400f32;

    // Create a new ImgBuf
    let mut imgbuf = image::ImageBuffer::new(img_side, img_side);

    // Create triangle vertices
    let mut vertices: [[f32; 2]; 3] = [[0f32, 0f32]; 3];
    for i in 0..vertices.len() {
        vertices[i][0] =
            (img_side as f32 / 2.) + (tri_size / 2.) * (f32::consts::PI * i as f32 * 2. / 3.).cos();
        vertices[i][1] =
            (img_side as f32 / 2.) + (tri_size / 2.) * (f32::consts::PI * i as f32 * 2. / 3.).sin();
    }
    for v in &vertices {
        imgbuf.put_pixel(v[0] as u32, v[1] as u32, image::Luma([255u8]));
    }

    // Iterate chaos game
    let mut rng = rand::weak_rng();
    let mut x = img_side as f32 / 2.;
    let mut y = img_side as f32 / 2.;
    for _ in 0..max_iterations {
        let choice = rng.gen_range(0, vertices.len());
        x = (x + vertices[choice][0]) / 2.;
        y = (y + vertices[choice][1]) / 2.;

        imgbuf.put_pixel(x as u32, y as u32, image::Luma([255u8]));
    }

    // Save image
    let fout = &mut File::create("fractal.png").unwrap();
    image::ImageLuma8(imgbuf).save(fout, image::PNG).unwrap();
}
