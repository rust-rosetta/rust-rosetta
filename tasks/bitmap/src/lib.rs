extern crate nom;
extern crate thiserror;
mod parser;

use std::default::Default;
use std::fmt;
use std::io::{BufWriter, Error, Write};
use std::ops::{Index, IndexMut};
use std::{fs::File, io::Read};
use thiserror::Error;

#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ImageFormat {
    P3,
    P6,
}

impl From<&str> for ImageFormat {
    fn from(i: &str) -> Self {
        match i.to_lowercase().as_str() {
            "p3" => ImageFormat::P3,
            "p6" => ImageFormat::P6,
            _ => unimplemented!("no other formats supported"),
        }
    }
}

impl fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImageFormat::P3 => {
                write!(f, "P3")
            }
            ImageFormat::P6 => {
                write!(f, "P6")
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum ImageError {
    #[error("File not found")]
    FileNotFound,
    #[error("File not readable")]
    FileNotReadable,
    #[error("Invalid header information")]
    InvalidHeader,
    #[error("Invalid information in the data block")]
    InvalidData,
    #[error("Invalid max color information")]
    InvalidMaxColor,
    #[error("File is incomplete")]
    IncompleteFile,
    #[error("unknown data store error")]
    Unknown,
}
pub struct Image {
    pub format: ImageFormat,
    pub width: usize,
    pub height: usize,
    pub data: Vec<Color>,
}

impl Image {
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            format: ImageFormat::P6,
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

    /// # Errors
    ///
    /// Will return `Error` if `filename` does not exist or the user does not have
    /// permission to write to it, or the write operation fails.
    pub fn write_ppm(&self, filename: &str) -> Result<(), Error> {
        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);
        writeln!(&mut writer, "{}", self.format)?;
        writeln!(&mut writer, "{} {} 255", self.width, self.height)?;
        match self.format {
            ImageFormat::P3 => {
                writer.write_all(
                    self.data
                        .iter()
                        .flat_map(|color| {
                            vec![
                                color.red.to_string(),
                                color.green.to_string(),
                                color.blue.to_string(),
                            ]
                        })
                        .collect::<Vec<String>>()
                        .join(" ")
                        .as_bytes(),
                )?;
            }
            ImageFormat::P6 => {
                writer.write_all(
                    &self
                        .data
                        .iter()
                        .flat_map(|color| vec![color.red, color.green, color.blue])
                        .collect::<Vec<u8>>(),
                )?;
            }
        }
        Ok(())
    }

    /// # Panics
    ///
    /// Panics if the format is not P6 or P3 PPM
    /// # Errors
    ///
    /// Will return `Error` if `filename` does not exist or the user does not have
    /// permission to read it or the read operation fails, or the file format does not
    /// match the specification
    pub fn read_ppm(filename: &str) -> Result<Image, ImageError> {
        let mut file = File::open(filename).map_err(|_| ImageError::FileNotFound)?;
        let mut data: Vec<u8> = Vec::new();
        file.read_to_end(&mut data)
            .map_err(|_| ImageError::FileNotReadable)?;

        let (i, format) = parser::parse_version(&data).map_err(|_| ImageError::InvalidHeader)?;
        let (i, (width, height, max_color)) =
            parser::parse_image_attributes(i).map_err(|_| ImageError::InvalidHeader)?;

        if max_color != 255 {
            return Err(ImageError::InvalidMaxColor);
        }

        let (_, data) = match format {
            ImageFormat::P3 => parser::parse_data_ascii(i).map_err(|_| ImageError::InvalidData)?,
            ImageFormat::P6 => parser::parse_data_binary(i).map_err(|_| ImageError::InvalidData)?,
        };

        if data.len() != height * width {
            return Err(ImageError::IncompleteFile);
        };

        Ok(Image {
            format,
            width,
            height,
            data,
        })
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
