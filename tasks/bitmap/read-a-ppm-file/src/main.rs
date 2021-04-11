use bitmap::Image;

// see read_ppm implementation in the bitmap library

pub fn main() {
    // read a PPM image, which was produced by the write-a-ppm-file task
    let mut image = Image::new(64, 64);
    image.read_ppm("./test_image.ppm").unwrap();

    println!("Read using simple parsing:");
    println!("Format: {:?}", image.format);
    println!("Dimensions: {} x {}", image.height, image.width);

    let image = Image::parse_ppm("./test_image.ppm").unwrap();

    println!("Read using nom parsing:");
    println!("Format: {:?}", image.format);
    println!("Dimensions: {} x {}", image.height, image.width);
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use bitmap::{Color, Image};
    use std::env;
    #[test]
    fn read_ppm() {
        let mut image = Image::new(2, 1);
        image[(0, 0)] = Color {
            red: 255,
            green: 0,
            blue: 0,
        };
        image[(1, 0)] = Color {
            red: 0,
            green: 255,
            blue: 0,
        };

        let fname = format!(
            "{}/test-{}.ppm",
            env::temp_dir().to_str().unwrap(),
            self::rand::random::<i32>(),
        );

        image.write_ppm(&fname).unwrap();

        let mut image = Image::new(0, 0);
        image.read_ppm(&fname).unwrap();

        assert_eq!(image.width, 2);
        assert_eq!(image.height, 1);
        assert_eq!(
            image.data,
            vec![
                Color {
                    red: 255,
                    green: 0,
                    blue: 0
                },
                Color {
                    red: 0,
                    green: 255,
                    blue: 0
                }
            ]
        )
    }
}
