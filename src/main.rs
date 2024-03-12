#![doc(html_root_url = "https://docs.rs/termris/0.1.0")]
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
pub struct View {
  /// term
  pub tm: PrayTerm
}

/// View
impl View {
  /// constructor
  pub fn new() -> Result<Self, Box<dyn Error>> {
    Ok(View{tm: PrayTerm::new(2)?})
  }
}

/// Termris
pub struct Termris {
  /// model
  pub m: Model,
  /// view
  pub v: View,
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

  /// proc
  pub fn proc(&mut self, rx: &mpsc::Receiver<Result<Event, std::io::Error>>) ->
    Result<bool, Box<dyn Error>> {
    // thread::sleep(self.m.ms);
    match rx.recv_timeout(self.m.ms) {
    Err(mpsc::RecvTimeoutError::Disconnected) => Err("Disconnected".into()),
    Err(mpsc::RecvTimeoutError::Timeout) => { // idle
      self.v.tm.wr(0, 47, 3, Rgb(192, 192, 192), Rgb(8, 8, 8),
        &format!("{:?}", self.t.elapsed()))?;
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
          self.v.tm.wr(0, 46, 3, Rgb(192, 192, 192), Rgb(8, 8, 8),
            &format!("mouse[{}, {}]", x, y))?;
          true
        },
        MouseEventKind::Down(MouseButton::Left) => {
          self.v.tm.wr(0, 45, 3, Rgb(192, 192, 192), Rgb(8, 8, 8),
            &format!("click[{}, {}]", x, y))?;
          true
        },
        _ => true
        }
      },
      Ok(Event::Resize(w, h)) => {
        self.v.tm.wr(0, 44, 3, Rgb(192, 192, 192), Rgb(8, 8, 8),
          &format!("resize[{}, {}]", w, h))?;
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
  tr.v.tm.wr(0, 0, 3, Rgb(255, 0, 0), Rgb(0, 255, 0), &"thread start".to_string())?;
  tr.v.tm.wr(0, 1, 3, Rgb(0, 255, 0), Rgb(0, 0, 255), &"main loop".to_string())?;
  tr.mainloop()?;
  tr.v.tm.wr(0, 48, 3, Rgb(0, 0, 255), Rgb(255, 0, 0), &"end".to_string())?;
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
