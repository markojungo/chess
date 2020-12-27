use std::fmt;
use std::default;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub enum Color {
  White,
  Black
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub enum Piece {
  None,
  Pawn(Color),
  Knight(Color),
  Bishop(Color),
  Rook(Color),
  Queen(Color),
  King(Color),
}

impl default::Default for Piece {
  fn default() -> Piece { Piece::None }
}

impl Piece {
  pub fn to_string(&self) -> String {
    format!("{}", self)
  }
}

impl fmt::Display for Piece {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", match self {
      Piece::Pawn(color) => match color {
        Color::White => "♙",
        Color::Black => "♟"
      },
      Piece::Knight(color) => match color {
        Color::White => "♘",
        Color::Black => "♞"
      },
      Piece::Bishop(color) => match color {
        Color::White => "♗",
        Color::Black => "♝"
      },
      Piece::Rook(color)   => match color {
        Color::White => "♖",
        Color::Black => "♜"
      },
      Piece::Queen(color)  => match color {
        Color::White => "♕",
        Color::Black => "♛"
      },
      Piece::King(color)   => match color {
        Color::White => "♔",
        Color::Black => "♚"
      },
      Piece::None => " "
    })
  }
}