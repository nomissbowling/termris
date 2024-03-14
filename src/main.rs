#![doc(html_root_url = "https://docs.rs/termris/0.3.1")]
//! termris terminal tetris for Rust
//!

use std::error::Error;
use std::time;
use std::sync::mpsc;

use crossterm::event::Event;
use crossterm::event::{KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::event::KeyCode::{self, Left, Down, Up, Right};
use crossterm::event::{MouseEvent, MouseEventKind, MouseButton};
use crossterm::style;

use prayterm::{PrayTerm, Rgb, NopColor};

use mvc_rs::View as MVCView;

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
  LightBlack = 0x00202020, LightBlack_ = 0xc0f0f0f0
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

/// Model
pub struct Model {
  /// timeout
  pub ms: time::Duration
}

/// Model
impl Model {
  /// constructor
  pub fn new() -> Self {
    Model{ms: time::Duration::from_millis(10)}
  }
}

/// View
pub struct View<T> {
  /// colors
  pub colors: Vec<T>,
  /// term
  pub tm: PrayTerm
}

/// trait MVCView for View
impl<T: NopColor + Clone> MVCView<T> for View<T> {
  /// wr
  fn wr(&mut self, x: u16, y: u16, st: u16,
    bgc: u16, fgc: u16, msg: &String) -> Result<(), Box<dyn Error>> {
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

/// Termris
pub struct Termris {
  /// model
  pub m: Model,
  /// view
  pub v: View<NColor>,
  /// time Instant
  pub t: time::Instant
}

/// trait Drop for Termris
impl Drop for Termris {
  /// destructor
  fn drop(&mut self) {
    self.v.tm.fin().expect("fin");
  }
}

/// Termris
impl Termris {
  /// constructor
  pub fn new() -> Result<Self, Box<dyn Error>> {
    let m = Model::new();
    let colors = [ // bgc fgc
      NColor::Black, NColor::Black_,
      NColor::Gray, NColor::Gray_,
      NColor::Red, NColor::Red_,
      NColor::Green, NColor::Green_,
      NColor::Blue, NColor::Blue_,
      NColor::Cyan, NColor::Cyan_,
      NColor::Magenta, NColor::Magenta_,
      NColor::Yellow, NColor::Yellow_,
      NColor::Orange, NColor::Orange_,
      NColor::Violet, NColor::Violet_,
      NColor::White, NColor::White_,
      NColor::LightRed, NColor::LightRed_,
      NColor::LightGreen, NColor::LightGreen_,
      NColor::LightBlue, NColor::LightBlue_,
      NColor::LightBlack, NColor::LightBlack_].to_vec();
    let mut s = Termris{m, v: View::new(colors)?, t: time::Instant::now()};
    s.v.tm.begin()?;
    Ok(s)
  }

  /// status
  pub fn status(&mut self, h: u16, st: u16, c: u16, s: &String) ->
    Result<(), Box<dyn Error>> {
    self.v.wr(0, self.v.tm.h - h, st, c, c + 1, s)?;
    Ok(())
  }

  /// stat
  pub fn stat(&mut self, h: u16, st: u16,
    bgc: impl NopColor, fgc: impl NopColor, s: &String) ->
    Result<(), Box<dyn Error>> {
    self.v.tm.wr(0, self.v.tm.h - h, st, bgc, fgc, s)?;
    Ok(())
  }

  /// proc
  pub fn proc(&mut self, rx: &mpsc::Receiver<Result<Event, std::io::Error>>) ->
    Result<bool, Box<dyn Error>> {
    // thread::sleep(self.m.ms);
    match rx.recv_timeout(self.m.ms) {
    Err(mpsc::RecvTimeoutError::Disconnected) => Err("Disconnected".into()),
    Err(mpsc::RecvTimeoutError::Timeout) => { // idle
      self.status(3, 3, 20, &format!("{:?}", self.t.elapsed()))?;
      Ok(true)
    },
    Ok(ev) => {
      Ok(match ev {
      Ok(Event::Key(k)) => {
        let f = match k {
        KeyEvent{kind: KeyEventKind::Press, state: _, code, modifiers} => {
          match (code, modifiers) {
          (KeyCode::Char('x'), KeyModifiers::ALT) => false,
          (KeyCode::Char('c'), KeyModifiers::CONTROL) => false,
          (KeyCode::Char('q'), _) => false,
          (KeyCode::Char('\x1b'), _) => false,
          (KeyCode::Esc, _) => false,
          _ => true // through down when kind == KeyEventKind::Press
          }
        },
        _ => true // through down when kind != KeyEventKind::Press
        };
        if !f { return Ok(false); }
        true
      },
      Ok(Event::Mouse(MouseEvent{kind, column: x, row: y, modifiers: _})) => {
        match kind {
        MouseEventKind::Moved => {
          self.status(4, 3, 18, &format!("mouse[{}, {}]", x, y))?;
          true
        },
        MouseEventKind::Down(MouseButton::Left) => {
          self.status(5, 3, 16, &format!("click[{}, {}]", x, y))?;
          true
        },
        _ => true
        }
      },
      Ok(Event::Resize(w, h)) => {
        self.status(6, 3, 2, &format!("resize[{}, {}]", w, h))?;
        true
      },
      _ => true
      })
    }
    }
  }

  /// mainloop
  pub fn mainloop(&mut self) -> Result<(), Box<dyn Error>> {
    let (_tx, rx) = self.v.tm.prepare_thread(self.m.ms)?;
    loop { if !self.proc(&rx)? { break; } }
    // handle.join()?;
    Ok(())
  }
}

pub fn main() -> Result<(), Box<dyn Error>> {
  let mut tr = Termris::new()?;
  tr.stat(50, 3, NColor::Red, NColor::Cyan, &"thread start".to_string())?;
  tr.stat(49, 3, NColor::Green, NColor::Magenta, &"main loop".to_string())?;
  tr.mainloop()?;
  tr.stat(2, 3, Rgb(0, 0, 255), Rgb(255, 255, 0), &"end".to_string())?;
  Ok(())
}

/// test with [-- --nocapture] or [-- --show-output]
#[cfg(test)]
mod tests {
  // use super::*;

  /// test a
  #[test]
  fn test_a() {
    assert_eq!(true, true);
  }
}
