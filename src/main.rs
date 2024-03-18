#![doc(html_root_url = "https://docs.rs/termris/3.4.2")]
//! termris terminal tetris for Rust
//!

use std::error::Error;
use std::time;
use std::sync::mpsc;

use crossterm::event::Event;
use crossterm::event::{KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::event::KeyCode::{self, Left, Down, Up, Right};
use crossterm::event::{MouseEvent, MouseEventKind, MouseButton};

use prayterm::{Rgb, NopColor};
use mvc_rs::TView;

pub mod packet;
use packet::Packet;

pub mod model;
use model::Model;

pub mod view;
use view::{NColor, View};

pub mod controller;
use controller::Controller;

pub mod stat;

/// Termris
pub struct Termris {
  /// view
  pub v: View<NColor>,
  /// controller
  pub c: Controller,
  /// time Instant
  pub t: time::Instant,
  /// time Duration
  pub d: time::Duration,
  /// timeout
  pub ms: time::Duration
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
    let m = Model::new(10, 20);
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
      NColor::LightBlack, NColor::LightBlack_,
      NColor::DarkGray, NColor::DarkGray_].to_vec();
    let v = View::new(colors)?;
    let c = Controller::new(m);
    let mut s = Termris{v, c,
      t: time::Instant::now(), d: time::Duration::new(0, 120_000_000), // ns
      ms: time::Duration::from_millis(10)};
    s.v.tm.begin()?;
    s.c.init(&mut s.v)?;
    Ok(s)
  }

  /// status
  pub fn status(&mut self, h: u16, st: u16, c: u16, msg: &String) ->
    Result<(), Box<dyn Error>> {
    self.v.wr(Packet{x: 0, y: self.v.tm.h - h, st, bgc: c, fgc: c + 1, msg})?;
    Ok(())
  }

  /// stat
  pub fn stat(&mut self, h: u16, st: u16,
    bgc: impl NopColor, fgc: impl NopColor, msg: &String) ->
    Result<(), Box<dyn Error>> {
    self.v.tm.wr(0, self.v.tm.h - h, st, bgc, fgc, msg)?;
    Ok(())
  }

  /// key
  pub fn key(&mut self, k: KeyEvent) -> bool {
    if k.kind != KeyEventKind::Press { return false; }
    let mut f = true;
    match k.code {
    Left | KeyCode::Char('h') => { self.c.proc_key(1); },
    Down | KeyCode::Char('j') => { self.c.proc_key(4); },
    Up | KeyCode::Char('k') => { self.c.proc_key(3); },
    Right | KeyCode::Char('l') => { self.c.proc_key(2); },
    KeyCode::Char(' ') => { self.c.proc_key(0); },
    _ => { f = false; }
    }
    f
  }

  /// proc
  pub fn proc(&mut self, rx: &mpsc::Receiver<Result<Event, std::io::Error>>) ->
    Result<bool, Box<dyn Error>> {
    // thread::sleep(self.ms);
    match rx.recv_timeout(self.ms) {
    Err(mpsc::RecvTimeoutError::Disconnected) => Err("Disconnected".into()),
    Err(mpsc::RecvTimeoutError::Timeout) => { // idle
      self.status(3, 3, 20, &format!("{:?}", self.t.elapsed()))?;
      self.c.refresh(&mut self.v)?;
      let u = time::Instant::now();
/*
      if u.duration_since(self.t) >= self.d {
        if !self.c.proc_idle() { return Ok(false); }
        self.t = u;
      }
*/
      if let Some(n) = u.checked_duration_since(self.t) {
        if n >= self.d {
          if !self.c.proc_idle() { return Ok(false); }
          self.t = u;
        }
      } else {
        ()
      }
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
        if self.key(k) { self.t -= self.d; }
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
    let (_tx, rx) = self.v.tm.prepare_thread(self.ms)?;
    loop { if !self.proc(&rx)? { break; } }
    self.c.fin(&mut self.v)?;
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
