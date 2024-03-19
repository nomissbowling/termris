//! controller
//!

use std::error::Error;

use crossterm::event::{KeyEvent, KeyEventKind};
use crossterm::event::KeyCode::{self, Left, Down, Up, Right};

use mvc_rs::TView;

use crate::model::Model;
use crate::stat::Stat;

/// Controller
pub struct Controller {
  /// model
  pub m: Model,
  /// status
  pub current: Stat
}

/// Controller
impl Controller {
  /// constructor
  pub fn new(m: Model) -> Self {
    Controller{m, current: Stat::new(0, 0, 0, 0)}
  }

  /// init
  pub fn init<T>(&mut self, _g: &mut impl TView<T>) ->
    Result<(), Box<dyn Error>> {
    self.m.init_board(&mut self.current);
    Ok(())
  }

  /// fin
  pub fn fin<T>(&mut self, g: &mut impl TView<T>) ->
    Result<(), Box<dyn Error>> {
    self.refresh(g)?;
    Ok(())
  }

  /// refresh
  pub fn refresh<T>(&mut self, g: &mut impl TView<T>) ->
    Result<(), Box<dyn Error>> {
    self.m.b.display_board(g)?;
    Ok(())
  }

  /// proc idle
  pub fn proc_idle(&mut self) -> bool {
    self.m.down_mino(&mut self.current) & 0x01 == 0
  }

  /// key
  pub fn key(&mut self, k: KeyEvent) -> bool {
    if k.kind != KeyEventKind::Press { return false; }
    let mut f = true;
    let k = match k.code {
    Left | KeyCode::Char('h') => 1,
    Down | KeyCode::Char('j') => 4,
    Up | KeyCode::Char('k') => 3,
    Right | KeyCode::Char('l') => 2,
    KeyCode::Char(' ') => 0,
    _ => { f = false; 0xff }
    };
    if f { if self.m.proc_key(&mut self.current, k) == 0 { f = false; } }
    f
  }

  /// speed
  pub fn speed(&self) -> u32 {
    self.m.f * 20 + 1
  }
}
