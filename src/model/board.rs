//! board
//!

use std::error::Error;

use mvc_rs::TView;
use crate::packet::Packet;

/// Board
pub struct Board {
  /// width
  pub w: u16,
  /// height
  pub h: u16,
  /// max width + 2
  pub mw: u16,
  /// max height + 5
  pub mh: u16,
  /// fld
  pub fld: Vec<Vec<u8>>
}

/// Board
impl Board {
  /// constructor
  pub fn new(w: u16, h: u16) -> Self {
    let mw = w + 2;
    let mh = h + 5;
    let fld = (0..mh).into_iter().map(|_r|
      (0..mw).into_iter().map(|_c|
          0).collect::<Vec<_>>()).collect::<Vec<_>>();
    Board{w, h, mw, mh, fld}
  }

  /// clear board
  pub fn clear_board(&mut self) {
    for r in self.fld.iter_mut() { for c in r { *c = 0; } }
  }

  /// get dot
  pub fn get_dot(&self, x: u16, y: u16) -> u8 {
    self.fld[y as usize][x as usize]
  }

  /// display board
  pub fn display_board<T>(&self, g: &mut impl TView<T>) ->
    Result<(), Box<dyn Error>> {
    for r in 0..self.mh {
      for c in 0..self.mw {
        let x = 2 + c * 2;
        let y = 2 + self.mh - 1 - r;
        let bgc = (self.get_dot(c, r) & 0x0f) as u16 * 2;
        let fgc = bgc + 1;
        let msg = &"  ".to_string();
        g.wr(Packet{x, y, st: 3, bgc, fgc, msg})?;
      }
    }
    Ok(())
  }

  /// draw dot
  pub fn draw_dot(&mut self, x: u16, y: u16, f: u8, c: u8) {
    self.fld[y as usize][x as usize] = (f << 4) | c;
  }

  /// draw box
  pub fn draw_box(&mut self) {
    for r in 0..self.mh {
      self.draw_dot(0, r, 1, 1);
      self.draw_dot(self.mw - 1, r, 1, 1);
    }
    for c in 0..self.mw {
      self.draw_dot(c, 0, 1, 1);
    }
  }

  /// draw dot copy
  pub fn draw_dot_copy(&mut self, dstx: u16, dsty: u16, srcx: u16, srcy: u16) {
    let p = self.get_dot(srcx, srcy);
    self.draw_dot(dstx, dsty, (p & 0xf0) >> 4, p & 0x0f);
  }
}
