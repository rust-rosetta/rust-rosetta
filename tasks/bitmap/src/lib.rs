use std::io::{BufReader, BufWriter, Error, ErrorKind, Write};
use std::ops::{Index, IndexMut};
use std::{default::Default, io::BufRead};
use std::{fs::File, io::Read};

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
        writer.write_all(
            &self
                .data
                .iter()
                .map(|color| vec![color.red, color.green, color.blue])
                .flatten()
                .collect::<Vec<u8>>(),
        )?;
        Ok(())
    }

    pub fn read_ppm(&mut self, filename: &str) -> Result<(), Error> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);

        let mut buf = String::new();
        reader.read_line(&mut buf)?;

        // works only with P6 (binary)
        assert!(buf == "P6\n");

        buf.clear();
        reader.read_line(&mut buf)?;

        // header parameters assumed to be separated by spaces
        let mut info_iter = buf.trim().split(' ');
        let width = info_iter
            .next()
            .unwrap_or("0")
            .parse::<usize>()
            .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid dimensions"))?;

        let height = info_iter
            .next()
            .unwrap_or("0")
            .parse::<usize>()
            .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid dimensions"))?;

        let max_color = info_iter
            .next()
            .unwrap_or("0")
            .parse::<usize>()
            .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid dimensions"))?;

        // check for invalid header values
        if width < 1 || height < 1 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid dimensions"));
        };

        if max_color != 255 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid header"));
        };

        // read pixel information
        let mut data = vec![0; height * width * 3];
        reader.read_exact(&mut data)?;

        self.height = height;
        self.width = width;

        // reconstruct Color structs from byte array
        self.data = data
            .chunks(3)
            .map(|c| Color {
                red: c[0],
                green: c[1],
                blue: c[2],
            })
            .collect();

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
    #[allow(clippy::no_effect)]
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
