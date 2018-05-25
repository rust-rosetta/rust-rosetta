extern crate image;
extern crate rand;

use std::cmp::min;
use std::env;
use std::path::Path;
use std::process;

use image::ColorType;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

fn help() {
    println!("Usage: brownian_tree <output_path> <mote_count> <edge_length>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut output_path = Path::new("out.png");
    let mut mote_count: u32 = 10_000;
    let mut width: usize = 512;
    let mut height: usize = 512;

    match args.len() {
        1 => {}
        4 => {
            output_path = Path::new(&args[1]);
            mote_count = args[2].parse::<u32>().unwrap();
            width = args[3].parse::<usize>().unwrap();
            height = width;
        }
        _ => {
            help();
            process::exit(0);
        }
    }

    assert!(width >= 2);

    // Base 1d array
    let mut field_raw = vec![0u8; width * height];
    populate_tree(&mut field_raw, width, height, mote_count);

    // Balance image for 8-bit grayscale
    let our_max = field_raw.iter().max().unwrap();
    let fudge = u8::max_value() / our_max;
    let balanced: Vec<u8> = field_raw.iter().map(|e| e * fudge).collect();

    match image::save_buffer(
        output_path,
        &balanced,
        width as u32,
        height as u32,
        ColorType::Gray(8),
    ) {
        Err(e) => println!("Error writing output image:\n{}", e),
        Ok(_) => println!("Output written to:\n{}", output_path.to_str().unwrap()),
    }
}

fn populate_tree(raw: &mut Vec<u8>, width: usize, height: usize, mc: u32) {
    // Vector of 'width' elements slices
    let mut field_base: Vec<_> = raw.chunks_mut(width).collect();
    // Addressable 2d vector
    let field = field_base.as_mut_slice();

    // Seed mote
    field[width / 2][height / 2] = 1;

    let x_spawn_range = Uniform::new(1, width - 1);
    let y_spawn_range = Uniform::new(1, height - 1);
    let mut rng = thread_rng();

    for i in 0..mc {
        if i % 100 == 0 {
            println!("{}", i)
        }

        // Spawn mote
        let mut x = rng.sample(x_spawn_range);
        let mut y = rng.sample(y_spawn_range);

        // Increment field value when motes spawn on top of the structure
        if field[x][y] > 0 {
            field[x][y] = min(field[x][y] + 1, u8::max_value()) as u8;
            continue;
        }

        loop {
            let contacts = field[x - 1][y - 1]
                + field[x][y - 1]
                + field[x + 1][y - 1]
                + field[x - 1][y]
                + field[x + 1][y]
                + field[x - 1][y + 1]
                + field[x][y + 1]
                + field[x + 1][y + 1];

            if contacts > 0 {
                field[x][y] = min(field[x][y] + 1, u8::max_value()) as u8;
                break;
            } else {
                let range = Uniform::new(-1, 2);
                let xw = rng.sample(range) + x as i32;
                let yw = rng.sample(range) + y as i32;
                if xw < 1 || xw >= (width as i32 - 1) || yw < 1 || yw >= (height as i32 - 1) {
                    // println!("wandered off");
                    break;
                }
                x = xw as usize;
                y = yw as usize;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::populate_tree;

    #[test]
    fn test_brownian_tree() {
        let width = 128;
        let height = 128;
        let mote_count = 1000;
        let mut field_raw = vec![0u8; width * height];
        populate_tree(&mut field_raw, width, height, mote_count);
        let our_max = field_raw.iter().max().unwrap();
        assert!(*our_max >= 1);
    }
}
