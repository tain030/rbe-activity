use std::fmt;

#[derive(Debug)]
struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let red = self.red;
    let green = self.green;
    let blue = self.blue;
    let rgb: usize = (red as usize * 65536) + (green as usize * 256) + blue as usize;
    write!(f, "RGB ({}, {}, {}) 0x{:X}",
      red, green, blue, rgb)
  }
}

fn main() {
  for color in [
      Color { red: 128, green: 255, blue: 90 },
      Color { red: 0, green: 3, blue: 254 },
      Color { red: 0, green: 0, blue: 0 },
  ] {
      // Switch this to use {} once you've added an implementation
      // for fmt::Display.
      println!("{}", color);
  }
}