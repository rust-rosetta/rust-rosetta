extern crate bitmap;

use bitmap::{Color, Image};
use std::fs::File;
use std::io;
use std::io::prelude::*;

trait PPMWritable {
    fn write_ppm(&self, filename: &str) -> Result<(), io::Error>;
}

impl PPMWritable for Image {
    fn write_ppm(&self, filename: &str) -> Result<(), io::Error> {
        let mut writer = File::create(filename)?;
        // let mut writer = BufWriter::new(file);
        writeln!(&mut writer, "P6")?;
        writeln!(&mut writer, "{} {} 255", self.width, self.height)?;
        for color in &(self.data) {
            for &channel in &[color.red, color.green, color.blue] {
                let ch = channel as u8;
                writer.write_all(&[ch])?;
            }
        }
        Ok(())
    }
}

pub fn main() {
    // write a PPM image, the left side of which is red, and the right side
    // of which is blue
    let mut image = Image::new(64, 64);
    image.fill(Color {
        red: 255,
        green: 0,
        blue: 0,
    });
    for y in 0..64 {
        for x in 32..64 {
            image[(x, y)] = Color {
                red: 0,
                green: 0,
                blue: 255,
            };
        }
    }
    image.write_ppm("./test_image.ppm").unwrap();
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use super::*;
    use io::BufReader;
    use std::env;

    #[test]
    fn write_ppm() {
        let mut image = Image::new(2, 1);
        image[(0, 0)] = Color {
            red: 1,
            green: 2,
            blue: 3,
        };
        image[(1, 0)] = Color {
            red: 4,
            green: 5,
            blue: 6,
        };
        let fname = format!(
            "{}/test-{}.ppm",
            env::temp_dir().to_str().unwrap(),
            self::rand::random::<i32>(),
        );
        // Can't use try! macro because we want to panic, not return.
        image.write_ppm(&fname).unwrap_or_else(|e| panic!(e));

        let file = File::open(&fname).unwrap();
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        assert_eq!(line, "P6\n");
        line = String::new();
        reader.read_line(&mut line).unwrap();
        assert_eq!(line, "2 1 255\n");
        let mut bytes = reader.bytes();
        assert_eq!(bytes.next().unwrap().unwrap(), 49);
        assert_eq!(bytes.next().unwrap().unwrap(), 50);
        assert_eq!(bytes.next().unwrap().unwrap(), 51);
        assert_eq!(bytes.next().unwrap().unwrap(), 52);
        assert_eq!(bytes.next().unwrap().unwrap(), 53);
        assert_eq!(bytes.next().unwrap().unwrap(), 54);
        assert!(bytes.next().is_none());
    }
}
