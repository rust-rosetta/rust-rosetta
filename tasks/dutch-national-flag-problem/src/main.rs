extern crate rand;

use rand::Rng;

// if you use an enum you will have to implement a lot of traits for it before you can
// iterate and sort it my proposal is to just use u8 and image a C like enum thus
// 0 = red
// 1 = white
// 2 = blue
//
// look up table
const COLOR: &'static [&'static str] = &["Red", "White", "Blue"];

fn check_sorted(color_array: &[u8]) -> bool {
    let mut test_color = 0;
    for color in color_array {
        if *color < test_color {
            return false;
        }
        test_color = *color;
    }
    true
}

fn color_to_string(color_array: &[u8]) -> String {
    color_array.iter().fold("".to_string(), |string, &x| string + COLOR[x as usize])
}

fn main() {

    let mut rng = rand::thread_rng();
    let mut color_array = [0u8; 20];

    for color in &mut color_array {
        *color = rng.gen_range(0, 3);
    }

    if check_sorted(&color_array) {
        println!("oops i generated a sorted array {}",
                 color_to_string(&color_array));
    } else {
        println!("random flag {}", color_to_string(&color_array));
    }

    color_array.sort();
    println!("a dutch flag {}", color_to_string(&color_array));

}

#[test]
fn test_dutch_national_flag() {
    let flag_array = [0, 1, 2];
    assert_eq!(true, check_sorted(&flag_array));

    let mut color_array = [2, 0, 1];
    assert_eq!(false, check_sorted(&color_array));
    color_array.sort();
    assert_eq!(flag_array, color_array);
}
