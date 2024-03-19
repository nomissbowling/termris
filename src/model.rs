//! model (mino, board)
//!

use rand;
use rand::Rng;

pub mod mino;
use mino::Mino;

pub mod board;
use board::Board;

use crate::stat::Stat;

/// Model
pub struct Model {
  /// minos
  pub minos: Vec<Mino>,
  /// fall
  pub f: u32,
  /// board
  pub b: Board
}

/// Model
impl Model {
  /// constructor
  pub fn new(w: u16, h: u16) -> Self {
    Model{minos: Mino::gen_minos(), f: 0, b: Board::new(w, h)}
  }

  /// put mino (default action = 0)
  pub fn put_mino(&mut self, s: &Stat, action: u16) -> u16 {
    let (t, c) = if action == 2 { (0, 0) } else { (2, 1 + s.typ) };
    if action != 2 && self.b.get_dot(s.x, s.y) & 0xf0 != 0 { return 0; }
    if action != 0 { self.b.draw_dot(s.x, s.y, t, c); }
    let m = &self.minos[s.typ as usize];
    for q in m.pos.iter() {
      let (mut dx, mut dy) = (q.x, q.y);
      let r = s.rotate % m.rotate;
      for _j in 0..r {
        let (nx, ny) = (dx, dy);
        (dx, dy) = (ny, -nx);
      }
      let (ux, uy) = ((s.x as i16 + dx) as u16, (s.y as i16 + dy) as u16);
      if action != 2 && self.b.get_dot(ux, uy) & 0xf0 != 0 { return 0; }
      if action != 0 { self.b.draw_dot(ux, uy, t, c); }
    }
    if action == 0 { self.put_mino(s, 1); }
    1
  }

  /// delete mino
  pub fn delete_mino(&mut self, s: &Stat) -> u16 {
    self.put_mino(s, 2)
  }

  /// i rand
  pub fn i_rand(m: u8) -> u8 {
    (rand::thread_rng().gen_range(0..m as u32 * 65536) % m as u32) as u8
  }

  /// new mino
  pub fn new_mino(&mut self) -> Stat {
    Stat{
      x: self.b.w / 2,
      y: self.b.h + 1,
      rotate: Self::i_rand(4),
      typ: Self::i_rand(self.minos.len() as u8 - 1) + 1} // skip minos[0]
  }

  /// init board
  pub fn init_board(&mut self, s: &mut Stat) {
    self.b.clear_board();
    self.b.draw_box();
    *s = self.new_mino();
    self.put_mino(s, 0);
  }

  /// over
  pub fn over(&mut self) {
    for r in self.b.fld.iter_mut() {
      for c in r {
        if *c & 0x20 != 0 { *c |= 0x0f; }
      }
    }
  }

  /// delete line
  pub fn delete_line(&mut self) -> u16 {
    let mut n = 0;
    for r in 1..self.b.h - 2 {
      let mut flg = true;
      while flg {
        for c in 1..self.b.w + 1 {
          if self.b.get_dot(c, r) == 0 { flg = false; }
        }
        if !flg { break; }
        n += 1;
        for j in r..self.b.h - 2 {
          for i in 1..self.b.w + 1 {
            self.b.draw_dot_copy(i, j, i, j + 1);
          }
        }
      }
    }
    n
  }

  /// down mino
  pub fn down_mino(&mut self, s: &mut Stat) -> u16 {
    self.delete_mino(s);
    s.y -= 1;
    if self.put_mino(s, 0) == 0 {
      s.y += 1;
      self.put_mino(s, 0);
      self.delete_line();
      *s = self.new_mino();
      if self.put_mino(s, 0) == 0 { self.over(); return 1; } // over
      self.f = 0;
      return 2; // floor
    }
    0 // fall
  }

  /// proc key
  pub fn proc_key(&mut self, s: &mut Stat, k: u8) -> u16 {
    let mut ret = 0;
    let mut n = Stat::new(s.x, s.y, s.rotate, s.typ);
    match k {
    0 => { ret = 1; }, // skip or force down
    1 => { n.x -= 1; },
    2 => { n.x += 1; },
    3 => { n.rotate += 1; },
    4 => { ret = 1; },
    _ => { ret = 0; }
    }
    if n.x != s.x || n.y != s.y || n.rotate != s.rotate {
      self.delete_mino(s);
      if self.put_mino(&n, 0) != 0 {
        *s = n;
      } else {
        self.put_mino(s, 0);
      }
    }
    if ret == 1 { self.f = 1; }
    ret
  }
}
