//! stat
//!

/// Stat
pub struct Stat {
  /// x
  pub x: u16,
  /// y
  pub y: u16,
  /// rotate
  pub rotate: u8,
  /// typ
  pub typ: u8
}

/// Stat
impl Stat {
  /// constructor
  pub fn new(x: u16, y: u16, rotate: u8, typ: u8) -> Self {
    Stat{x, y, rotate, typ}
  }
}
