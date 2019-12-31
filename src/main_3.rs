//Add an implementation of the fmt::Display trait for the Color struct above so that the output displays as:
//
//RGB (128, 255, 90) 0x80FF5A
//RGB (0, 3, 254) 0x0003FE
//RGB (0, 0, 0) 0x000000

//Two hints if you get stuck:
//You may need to list each color more than once,
//You can pad with zeros to a width of 2 with :02.

use std::fmt::{Display, Formatter, Error};

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(f, "{}:{:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// attention!!!!!
impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "RGB ({0}, {1}, {2}) 0x{0:02X}{1:02X}{2:02X}", self.red, self.green, self.blue)
    }
}

fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
//        println!("{:?}", *color);
        println!("{}", *color);
    }
}