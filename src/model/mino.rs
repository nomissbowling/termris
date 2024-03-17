//! mino (tetra mino)
//!

/// Pos
pub struct Pos {
  /// x
  pub x: i16,
  /// y
  pub y: i16
}

/// Pos
impl Pos {
  /// constructor
  pub fn new(x: i16, y: i16) -> Self {
    Pos{x, y}
  }
}

/// Mino
pub struct Mino {
  /// rotate
  pub rotate: u8,
  /// pos (4 - 1)
  pub pos: Vec<Pos>
}

/// Mino
impl Mino {
  /// constructor
  pub fn new(rotate: u8, p: [[i16; 2]; 3]) -> Self {
    Mino{rotate, pos: p.iter().map(|p| Pos{x: p[0], y: p[1]}).collect()}
  }

  /// gen minos
  pub fn gen_minos() -> Vec<Mino> {
    vec![
      Mino::new(1, [[0, 0], [0, 0], [0, 0]]), // null (skip minos[0])
      Mino::new(2, [[0, -1], [0, 1], [0, 2]]), // tetris
      Mino::new(4, [[0, -1], [0, 1], [1, 1]]), // L1
      Mino::new(4, [[0, -1], [0, 1], [-1, 1]]), // L2
      Mino::new(2, [[0, -1], [1, 0], [1, 1]]), // key1
      Mino::new(2, [[0, -1], [-1, 0], [-1, 1]]), // key2
      Mino::new(1, [[0, 1], [1, 0], [1, 1]]), // square
      Mino::new(4, [[0, -1], [1, 0], [-1, 0]]) // T
    ]
  }
}
