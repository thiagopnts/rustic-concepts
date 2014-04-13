#[deriving(Show, FromPrimitive)]
pub enum Color {
  Black = 0,
  Blue  = 1,
  Green = 2,
  Cyan = 3,
  Red = 4,
  Pink = 5,
  Brown = 6,
  LightGray = 7,
  DarkGray = 8,
  LightBlue = 9,
  LightGreen = 10,
  LightCyan = 11,
  LightRed = 12,
  LightPink = 13,
  Yellow = 14,
  White = 15,
}

struct Char {
  pub char: u8,
  flags: u8, // 4 bits for foreground and 4 bits for background
}

impl Char {
  pub fn new(c: char, fg: Color, bg: Color) -> Char {
    Char { char: c as u8, flags: fg as u8 | (bg as u8 << 4) }
  }

  pub fn background(&self) -> Color {
    match FromPrimitive::from_u8(self.flags >> 4 as u8) {
      Some(color) => color,
      None => Black
    }
  }

  pub fn foreground(&self) -> Color {
    match FromPrimitive::from_u8(self.flags & 0x0F as u8) {
      Some(color) => color,
      None => Black
    }
  }
}

fn main() {
  let c = Char::new('f', Green, Black);
  println!("foreground: {}, background: {}", c.foreground(), c.background());
}

