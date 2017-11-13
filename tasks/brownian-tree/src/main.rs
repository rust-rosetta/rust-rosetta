extern crate image;
extern crate rand;

use image::ColorType;
use rand::distributions::{IndependentSample, Range};
use std::cmp::{min, max};
use std::env;
use std::path::Path;
use std::process;

fn help() {
    println!("Usage: brownian_tree <output_path> <mote_count> <edge_length>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut output_path = Path::new("out.png");
    let mut mote_count: u32 = 10000;
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
    let our_max = field_raw.iter().fold(0u8, |champ, e| max(champ, *e));
    let fudge = std::u8::MAX / our_max;
    let balanced: Vec<u8> = field_raw.iter().map(|e| e * fudge).collect();

    match image::save_buffer(output_path,
                             &balanced,
                             width as u32,
                             height as u32,
                             ColorType::Gray(8)) {
        Err(e) => println!("Error writing output image:\n{}", e),
        Ok(_) => println!("Output written to:\n{}", output_path.to_str().unwrap()),
    }
}


fn populate_tree(raw: &mut Vec<u8>, width: usize, height: usize, mc: u32) {
    // Vector of 'width' elements slices
    let mut field_base: Vec<_> = raw.as_mut_slice().chunks_mut(width).collect();
    // Addressable 2d vector
    let field = field_base.as_mut_slice();

    // Seed mote
    field[width / 2][height / 2] = 1;

    let walk_range = Range::new(-1i32, 2i32);
    let x_spawn_range = Range::new(1usize, width - 1);
    let y_spawn_range = Range::new(1usize, height - 1);
    let mut rng = rand::thread_rng();

    for i in 0..mc {
        if i % 100 == 0 {
            println!("{}", i)
        }

        // Spawn mote
        let mut x = x_spawn_range.ind_sample(&mut rng);
        let mut y = y_spawn_range.ind_sample(&mut rng);

        // Increment field value when motes spawn on top of the structure
        if field[x][y] > 0 {
            field[x][y] = min(field[x][y] as u32 + 1, std::u8::MAX as u32) as u8;
            continue;
        }

        loop {
            let contacts = field[x - 1][y - 1] + field[x][y - 1] + field[x + 1][y - 1] +
                           field[x - 1][y] + field[x + 1][y] +
                           field[x - 1][y + 1] + field[x][y + 1] +
                           field[x + 1][y + 1];

            if contacts > 0 {
                field[x][y] = min(field[x][y] as u32 + 1, std::u8::MAX as u32) as u8;
                break;
            } else {
                let xw = walk_range.ind_sample(&mut rng) + x as i32;
                let yw = walk_range.ind_sample(&mut rng) + y as i32;
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
    use std::cmp::max;

    #[test]
    fn test_brownian_tree() {
        let width = 128;
        let height = 128;
        let mote_count = 1000;
        let mut field_raw = vec![0u8; width * height];
        populate_tree(&mut field_raw, width, height, mote_count);
        let our_max = field_raw.iter().fold(0u8, |champ, e| max(champ, *e));
        assert!(our_max >= 1);
    }
}
