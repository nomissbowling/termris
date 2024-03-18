//! controller
//!

use std::error::Error;

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
    self.m.down_mino(&mut self.current) == 0
  }

  /// proc key
  pub fn proc_key(&mut self, k: u8) -> u16 {
    self.m.proc_key(&mut self.current, k)
  }
}
