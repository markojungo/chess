use crate::piece::{Piece, Color};
use std::fmt;
use std::default;
// pub struct Square {
//   piece: Piece
// }

pub struct Board {
  pieces: [[Piece; 8] ; 8],
}

// pub enum Status {
//   Ongoing,
//   Stalemate,
//   Checkmate
// }

// Set up default board.
impl default::Default for Board {
  fn default() -> Board {
    let mut b = Board::new();
    b.set_piece(Piece::Rook(Color::Black), (0, 0));
    b.set_piece(Piece::Knight(Color::Black), (0, 1));
    b.set_piece(Piece::Bishop(Color::Black), (0, 2));
    b.set_piece(Piece::Queen(Color::Black), (0, 3));
    b.set_piece(Piece::King(Color::Black), (0, 4));
    b.set_piece(Piece::Bishop(Color::Black), (0, 5));
    b.set_piece(Piece::Knight(Color::Black), (0, 6));
    b.set_piece(Piece::Rook(Color::Black), (0, 7));
    for i in 0..8 { b.set_piece(Piece::Pawn(Color::Black), (1, i)) }

    b.set_piece(Piece::Rook(Color::White), (7, 0));
    b.set_piece(Piece::Knight(Color::White), (7, 1));
    b.set_piece(Piece::Bishop(Color::White), (7, 2));
    b.set_piece(Piece::Queen(Color::White), (7, 3));
    b.set_piece(Piece::King(Color::White), (7, 4));
    b.set_piece(Piece::Bishop(Color::White), (7, 5));
    b.set_piece(Piece::Knight(Color::White), (7, 6));
    b.set_piece(Piece::Rook(Color::White), (7, 7));
    for i in 0..8 { b.set_piece(Piece::Pawn(Color::Black), (6, i)) }

    b
  }
}

impl fmt::Display for Board {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    Ok(for row in &self.pieces {
      for p in row {
        write!(f, "{} ", p)?;
      }
      write!(f, "\n")?;
    })
  }
}

impl Board {
  pub fn new() -> Board {
    Board {
      pieces: [[Piece::default(); 8] ; 8]
    }
  }

  // Add a piece onto the Board
  // Will replace an existing piece
  pub fn set_piece(&mut self, piece: Piece, pos: (usize, usize)) {
    self.pieces[pos.0][pos.1] = piece;
  }

  // Get piece at board location
  pub fn get_piece(&self, pos: (usize, usize)) -> Piece {
    self.pieces[pos.0][pos.1]
  }

  // Remove piece at position
  // If there is no piece at the position, return None
  // Else, return the piece
  pub fn remove_piece(&mut self, pos: (usize, usize)) -> Piece {
    let p = self.pieces[pos.0][pos.1];
    self.pieces[pos.0][pos.1] = Piece::None;
    p
  }
}

#[cfg(test)]
mod tests {
  use crate::board::{Board};
  use crate::piece::{Piece, Color};

  #[test]
  fn board_remove() {
    let mut b = Board::default();
    assert_eq!(b.remove_piece((0,0)), Piece::Rook(Color::Black));
  }
}