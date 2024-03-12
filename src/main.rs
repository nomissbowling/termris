#![doc(html_root_url = "https://docs.rs/termris/0.1.1")]
//! termris terminal tetris for Rust
//!

use std::error::Error;
use std::time;
use std::sync::mpsc;

use crossterm::event::Event;
use crossterm::event::{KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::event::KeyCode::{self, Left, Down, Up, Right};
use crossterm::event::{MouseEvent, MouseEventKind, MouseButton};
use crossterm::style::Color;

use prayterm::{PrayTerm, Rgb, NopColor};

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

/// View
impl<T: NopColor + Clone> View<T> {
  /// constructor
  pub fn new() -> Result<Self, Box<dyn Error>> {
    Ok(View{colors: vec![], tm: PrayTerm::new(2)?})
  }
}

/// Termris
pub struct Termris {
  /// model
  pub m: Model,
  /// view
  pub v: View<Rgb>,
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
    let mut s = Termris{m, v: View::new()?, t: time::Instant::now()};
    s.v.tm.begin()?;
    Ok(s)
  }

  /// status
  pub fn status(&mut self, h: u16, st: u16,
    bgc: impl NopColor, fgc: impl NopColor, s: &str) ->
    Result<(), Box<dyn Error>> {
    self.v.tm.wr(0, self.v.tm.h - h, st, bgc, fgc, &s.to_string())?;
    Ok(())
  }

  /// proc
  pub fn proc(&mut self, rx: &mpsc::Receiver<Result<Event, std::io::Error>>) ->
    Result<bool, Box<dyn Error>> {
    // thread::sleep(self.m.ms);
    match rx.recv_timeout(self.m.ms) {
    Err(mpsc::RecvTimeoutError::Disconnected) => Err("Disconnected".into()),
    Err(mpsc::RecvTimeoutError::Timeout) => { // idle
      self.status(3, 3, Rgb(192, 192, 192), Rgb(8, 8, 8),
        format!("{:?}", self.t.elapsed()).as_str())?;
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
          self.status(4, 3, Rgb(192, 192, 192), Rgb(8, 8, 8),
            format!("mouse[{}, {}]", x, y).as_str())?;
          true
        },
        MouseEventKind::Down(MouseButton::Left) => {
          self.status(5, 3, Rgb(192, 192, 192), Rgb(8, 8, 8),
            format!("click[{}, {}]", x, y).as_str())?;
          true
        },
        _ => true
        }
      },
      Ok(Event::Resize(w, h)) => {
        self.status(6, 3, Rgb(192, 192, 192), Rgb(8, 8, 8),
          format!("resize[{}, {}]", w, h).as_str())?;
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
  tr.status(50, 3, Color::Red, Color::Green, "thread start")?;
  tr.status(49, 3, Color::Green, Color::Blue, "main loop")?;
  tr.mainloop()?;
  tr.status(2, 3, Color::Blue, Color::Red, "end")?;
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
