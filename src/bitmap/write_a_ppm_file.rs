// http://rosettacode.org/wiki/Bitmap/Write_a_PPM_file

use std::path::Path;
use std::io::Write;
use std::fs::File;

pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

pub struct PPM {
    height: u32,
    width: u32,
    data: Vec<u8>,
}

impl PPM {
    pub fn new(height: u32, width: u32) -> PPM {
        let size = 3 * height * width;
        let buffer = vec![0; size as usize];
        PPM {
            height: height,
            width: width,
            data: buffer,
        }
    }

    fn buffer_size(&self) -> u32 {
        3 * self.height * self.width
    }

    fn get_offset(&self, x: u32, y: u32) -> Option<usize> {
        let offset = (y * self.width * 3) + (x * 3);
        if offset < self.buffer_size() {
            Some(offset as usize)
        } else {
            None
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<RGB> {
        match self.get_offset(x, y) {
            Some(offset) => {
                let r = self.data[offset];
                let g = self.data[offset + 1];
                let b = self.data[offset + 2];
                Some(RGB { r: r, g: g, b: b })
            }
            None => None,
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: RGB) -> bool {
        match self.get_offset(x, y) {
            Some(offset) => {
                self.data[offset] = color.r;
                self.data[offset + 1] = color.g;
                self.data[offset + 2] = color.b;
                true
            }
            None => false,
        }
    }

    pub fn write_file(&self, filename: &str) -> std::io::Result<()> {
        let path = Path::new(filename);
        let mut file = try!(File::create(&path));
        let header = format!("P6 {} {} 255\n", self.width, self.height);
        try!(file.write(header.as_bytes()));
        try!(file.write(&self.data));
        Ok(())
    }
}

fn main() {
    let ppm = PPM::new(400, 300);
    ppm.write_file("output.ppm").unwrap();
}
