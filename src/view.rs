//! view (PrayTerm)
//!

use std::error::Error;

use crossterm::style;

use prayterm::{PrayTerm, NopColor};
use mvc_rs::{TPacket, TView};

/// NColor ARGB bgc fgc
#[derive(Debug, Clone)]
pub enum NColor {
  Black = 0x00000000, Black_ = 0xc0ffffff,
  Gray = 0x00a0a0a0, Gray_ = 0xc0404040,
  Red = 0x00f04040, Red_ = 0xc040f0f0,
  Green = 0x0040f040, Green_ = 0xc0f040f0,
  Blue = 0x004040f0, Blue_ = 0xc0f0f040,
  Cyan = 0x0040f0f0, Cyan_ = 0xc0f04040,
  Magenta = 0x00f040f0, Magenta_ = 0xc040f040,
  Yellow = 0x00f0f040, Yellow_ = 0xc04040f0,
  Orange = 0x00f0c040, Orange_ = 0xc040c0f0,
  Violet = 0x00c040f0, Violet_ = 0xc0c0f040,
  White = 0x00f0f0f0, White_ = 0xc0202020,
  LightRed = 0x00f02060, LightRed_ = 0xc060f020,
  LightGreen = 0x0060f020, LightGreen_ = 0xc02060f0,
  LightBlue = 0x002060f0, LightBlue_ = 0xc0f02060,
  LightBlack = 0x00202020, LightBlack_ = 0xc0f0f0f0,
  DarkGray = 0x00606060, DarkGray_ = 0xc0c0c0c0
}

/// trait NopColor for NColor
impl NopColor for NColor {
  /// nop
  fn nop(&self) -> style::Color {
    let bgra = (0..4).into_iter().map(|i|
      (self.clone() as u32 >> (8 * i)) as u8 & 0x0ff).collect::<Vec<_>>();
    style::Color::Rgb{r: bgra[2], g: bgra[1], b: bgra[0]}
  }
}

/// View
pub struct View<T> {
  /// colors
  pub colors: Vec<T>,
  /// term
  pub tm: PrayTerm
}

/// trait TView for View
impl<T: NopColor + Clone> TView<T> for View<T> {
  /// wr
  fn wr(&mut self, p: impl TPacket) -> Result<(), Box<dyn Error>> {
    let v = p.to_vec();
    let (x, y, st, bgc, fgc) = (v[0], v[1], v[2], v[3], v[4]);
    let msg = &p.as_str().to_string();
    // let msg = &String::from_utf8(p.as_bytes().to_vec())?;
    self.tm.wr(x, y, st, self.col(bgc), self.col(fgc), msg)?;
    Ok(())
  }
  /// reg
  fn reg(&mut self, c: Vec<T>) -> () {
    self.colors = c;
  }
  /// col
  fn col(&self, n: u16) -> T {
    self.colors[n as usize].clone()
  }
}

/// View
impl<T: NopColor + Clone> View<T> {
  /// constructor
  pub fn new(colors: Vec<T>) -> Result<Self, Box<dyn Error>> {
    Ok(View{colors, tm: PrayTerm::new(2)?})
  }
}
