// Implements http://rosettacode.org/wiki/Write_ppm_file
#![allow(unstable)]
use std::io::{File, BufferedWriter, IoResult};
use bitmap::Image;
mod bitmap;

trait PPMWritable {
    fn write_ppm(&self, filename: &str) -> IoResult<()>;
}

impl PPMWritable for Image {
    fn write_ppm(&self, filename: &str) -> IoResult<()> {
        let file = File::create(&Path::new(filename));
        let mut writer = BufferedWriter::new(file);
        try!(writer.write_line("P6"));
        try!(write!(&mut writer, "{} {} {}\n", self.width, self.height, 255us));
        for color in self.data.iter() {
            for channel in [color.red, color.green, color.blue].iter() {
                try!(writer.write_u8(*channel));
            }
        }
        Ok(())
    }
}

#[cfg(not(test))]
pub fn main() {
    use bitmap::Color;

    // write a PPM image, the left side of which is red, and the right side
    // of which is blue
    let mut image = Image::new(64, 64);
    image.fill(Color { red: 255, green: 0, blue: 0 });
    for y in (0us..64) {
        for x in (32us..64) {
            image[(x, y)] = Color { red: 0, green: 0, blue: 255 };
        }
    }
    image.write_ppm("./test_image.ppm").unwrap();
}

#[cfg(test)]
mod test {
    use bitmap::{Color, Image};
    use std::io::{File, BufferedReader};
    use std::rand;
    use std::rand::Rng;
    use std::os;

    #[test]
    fn write_ppm() {
        let mut image = Image::new(2,1);
        image[(0, 0)] = Color { red: 1, green: 2, blue: 3 };
        image[(1, 0)] = Color { red: 4, green: 5, blue: 6 };
        let fname = format!("{}/test-{}.ppm", os::tmpdir().as_str().unwrap(), rand::thread_rng().gen::<i32>());
        // Can't use try! macro because we want to panic, not return.
        match image.write_ppm(fname.as_slice()) {
            Ok(_) => {},
            Err(e) => panic!(e)
        }

        let file = File::open(&Path::new(fname.as_slice()));
        let mut reader = BufferedReader::new(file);
        assert_eq!(reader.read_line().unwrap(), "P6\n");
        assert_eq!(reader.read_line().unwrap(), "2 1 255\n");
        assert_eq!(reader.read_byte().unwrap(), 1);
        assert_eq!(reader.read_byte().unwrap(), 2);
        assert_eq!(reader.read_byte().unwrap(), 3);
        assert_eq!(reader.read_byte().unwrap(), 4);
        assert_eq!(reader.read_byte().unwrap(), 5);
        assert_eq!(reader.read_byte().unwrap(), 6);
        assert!(reader.read_byte().is_err());

    }
}
