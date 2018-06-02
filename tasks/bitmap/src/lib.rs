use std::default::Default;
use std::fs::File;
use std::io::{BufWriter, Error, Write};
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Color>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![Color::default(); width * height],
        }
    }

    pub fn fill(&mut self, color: Color) {
        for elem in &mut self.data {
            *elem = color;
        }
    }

    pub fn write_ppm(&self, filename: &str) -> Result<(), Error> {
        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);
        writeln!(&mut writer, "P6")?;
        writeln!(&mut writer, "{} {} 255", self.width, self.height)?;
        for color in &self.data {
            for &channel in &[color.red, color.green, color.blue] {
                write!(&mut writer, "{}", channel as u8)?;
            }
        }
        Ok(())
    }
}

impl Index<(usize, usize)> for Image {
    type Output = Color;

    fn index(&self, (x, y): (usize, usize)) -> &Color {
        &self.data[x + y * self.width]
    }
}

impl IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Color {
        &mut self.data[x + y * self.width]
    }
}

#[allow(dead_code)]
pub fn main() {
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
            let ch = if color.red + color.green + color.blue == 0 {
                '#'
            } else {
                '.'
            };
            print!("{}", ch);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn out_of_bounds() {
        let image = Image::new(10, 10);
        image[(10, 11)];
    }

    #[test]
    fn getting() {
        let image = Image::new(3, 4);
        for x in 0..3 {
            for y in 0..4 {
                assert_eq!(image[(x, y)], Default::default());
            }
        }
    }

    #[test]
    fn setting() {
        let mut image = Image::new(3, 3);
        image[(0, 0)] = Color {
            red: 1,
            green: 1,
            blue: 1,
        };
        assert_eq!(
            image[(0, 0)],
            Color {
                red: 1,
                green: 1,
                blue: 1,
            }
        );
    }

    #[test]
    fn filling() {
        let mut image = Image::new(4, 3);
        let fill = Color {
            red: 3,
            green: 2,
            blue: 5,
        };
        image.fill(fill);
        for x in 0..4 {
            for y in 0..3 {
                assert_eq!(image[(x, y)], fill);
            }
        }
    }
}
