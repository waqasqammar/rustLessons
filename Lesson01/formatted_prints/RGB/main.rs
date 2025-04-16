use std::fmt;

// Define Color struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// implement fmt::Display trait 
impl fmt::Display for Color {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rgb = (self.red as u32 * 65536) + (self.green as u32 * 255) + (self.blue as u32);
        write!(f, "RGB ({}, {}, {}) {rgb:#08X}", self.red, self.green, self.blue)
    }
}

fn main () {
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ] {
        println!("{}", color);
    }
}