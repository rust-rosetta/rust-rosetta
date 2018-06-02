extern crate bitmap;

use bitmap::{Color, Image};

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
            let color = image[(x, y)];
            let ch = if color.red.wrapping_add(color.green).wrapping_add(color.blue) == 0 {
                '#'
            } else {
                '.'
            };
            print!("{}", ch);
        }
        println!();
    }
}
