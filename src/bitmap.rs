// Implements http://rosettacode.org/wiki/Basic_bitmap_storage
#![feature(old_io)]
#![feature(old_path)]

use std::default::Default;
use std::old_io::{File, BufferedWriter, IoResult};
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Color>
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            width: width,
            height: height,
            data: ::std::iter::repeat(Default::default())
				.take(width*height).collect(),
        }
    }

    pub fn fill(&mut self, color: Color) {
        for elem in self.data.iter_mut() {
            *elem = color;
        }
    }

    pub fn write_ppm(&self, filename: &str) -> IoResult<()> {
        let file = File::create(&Path::new(filename));
        let mut writer = BufferedWriter::new(file);
        try!(writer.write_line("P6"));
        try!(write!(&mut writer, "{} {} {}\n", self.width, self.height, 255));
        for color in &(self.data) {
            for channel in &[color.red, color.green, color.blue] {
                try!(writer.write_u8(*channel));
            }
        }
        Ok(())
    }
}

impl Index<(usize, usize)> for Image {
    type Output=Color;

    fn index<'a>(&'a self, &(x, y): &(usize, usize)) -> &'a Color {
        &self.data[x + y*self.width]
    }
}

impl IndexMut<(usize, usize)> for Image {
    fn index_mut<'a>(&'a mut self, &(x, y): &(usize, usize)) -> &'a mut Color {
        & mut self.data[x + y*self.width]
    }
}

#[cfg(not(test))]
#[allow(dead_code)]
pub fn main() {
    let mut image = Image::new(10, 10);

    for y in 0..10 {
        for x in 5..10 {
            image[(x,y)] = Color { red: 255, green: 255, blue: 255 };
        }
    }

    for y in 0..10 {
        for x in 0..10 {
            if image[(x,y)].red + image[(x,y)].green + image[(x,y)].blue == 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

#[cfg(test)]
mod test {
    use super::{Color, Image};
    use std::default::Default;

    #[test]
    #[should_fail]
    #[ignore(unused_variable)]
    fn out_of_bounds() {
        let image = Image::new(10, 10);
        let _ = image[(10, 11)];
        assert!(false);
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
        image[(0,0)] = Color { red: 1, green: 1, blue: 1 };
        assert_eq!(image[(0,0)], Color { red: 1, green: 1, blue: 1});
    }

    #[test]
    fn filling() {
        let mut image = Image::new(4, 3);
        let fill = Color { red: 3, green: 2, blue: 5};
        image.fill(fill);
        for x in 0..4 {
            for y in 0..3 {
                assert_eq!(image[(x, y)], fill);
            }
        }
    }
}
