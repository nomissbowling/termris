//! model (mino, board)
//!

use std::time;

use rand;
use rand::Rng;

pub mod mino;
use mino::Mino;

pub mod board;
use board::Board;

/// Stat
#[derive(Debug, Clone)]
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

/// Model
pub struct Model {
  /// minos
  pub minos: Vec<Mino>,
  /// board
  pub b: Board,
  /// status
  pub current: Stat,
  /// timeout
  pub ms: time::Duration
}

/// Model
impl Model {
  /// constructor
  pub fn new(w: u16, h: u16) -> Self {
    let minos = Mino::gen_minos();
    let b = Board::new(w, h);
    let current = Stat::new(0, 0, 0, 0);
    Model{minos, b, current, ms: time::Duration::from_millis(10)}
  }

  /// put mino (default action = 0)
  pub fn put_mino(&mut self, s: Option<&Stat>, action: u16) -> u16 {
    let o = self.current.clone(); // to keep instance
    let s = if let Some(s) = s { s } else { &o };
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
    if action == 0 { self.put_mino(Some(s), 1); }
    1
  }

  /// delete mino
  pub fn delete_mino(&mut self, s: Option<&Stat>) -> u16 {
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
  pub fn init_board(&mut self) {
    self.b.clear_board();
    self.b.draw_box();
    self.current = self.new_mino();
    self.put_mino(None, 0);
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
  pub fn delete_line(&mut self) {
    for r in 1..self.b.h - 2 {
      let mut flg = true;
      while flg {
        for c in 1..self.b.w + 1 {
          if self.b.get_dot(c, r) == 0 { flg = false; }
        }
        if !flg { break; }
        for j in r..self.b.h - 2 {
          for i in 1..self.b.w + 1 {
            self.b.draw_dot_copy(i, j, i, j + 1);
          }
        }
      }
    }
  }

  /// down mino
  pub fn down_mino(&mut self) -> u16 {
    self.delete_mino(None);
    self.current.y -= 1;
    if self.put_mino(None, 0) == 0 {
      self.current.y += 1;
      self.put_mino(None, 0);
      self.delete_line();
      self.current = self.new_mino();
      if self.put_mino(None, 0) == 0 { self.over(); return 1; }
    }
    0
  }

  /// proc key
  pub fn proc_key(&mut self, k: u8) -> u16 {
    let mut ret = 0;
    let mut n = self.current.clone();
    match k {
    0 => { ret = 1; }, // skip or force down
    1 => { n.x -= 1; },
    2 => { n.x += 1; },
    3 => { n.rotate += 1; },
    4 => { ret = 1; },
    _ => { ret = 0; }
    }
    let s = &self.current;
    if n.x != s.x || n.y != s.y || n.rotate != s.rotate {
      self.delete_mino(None);
      if self.put_mino(Some(&n), 0) != 0 {
        self.current = n;
      } else {
        self.put_mino(None, 0);
      }
    }
    ret
  }
}
