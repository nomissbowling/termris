//! packet
//!

use mvc_rs::TPacket;

/// Packet
pub struct Packet<'a> {
  /// x
  pub x: u16,
  /// y
  pub y: u16,
  /// style
  pub st: u16,
  /// bgc abstract id
  pub bgc: u16,
  /// fgc abstract id
  pub fgc: u16,
  /// msg
  pub msg: &'a String
}

/// trait TPacket for Packet
impl TPacket for Packet<'_> {
  /// to_vec
  fn to_vec(&self) -> Vec<u16> {
    vec![self.x, self.y, self.st, self.bgc, self.fgc]
  }
  /// as_bytes
  fn as_bytes(&self) -> &[u8] {
    self.msg.as_bytes()
  }
  /// as_str
  fn as_str(&self) -> &str {
    self.msg.as_str()
  }
}
