extern crate bitmap;

use bitmap::{Image, Color};

fn main() {
    let mut image = Image::new(10, 10);

    for y in 0..10 {
        for x in 5..10 {
            image[(x, y)] = Color {
                red: 255,
                green: 255,
                blue: 255,
            };
        }
    }

    for y in 0..10 {
        for x in 0..10 {
            if image[(x, y)].red + image[(x, y)].green + image[(x, y)].blue == 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}
