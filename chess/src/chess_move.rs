use crate::square::Square;
use crate::piece::Piece;

#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Default, Debug, Hash)]
struct ChessMove {
  source: Square,
  dest: Square,
  promotion: Option<Piece>,
}

impl ChessMove {
  pub fn new(source: Square, dest: Square, promotion: Option<Piece>) -> ChessMove {
    ChessMove {
      source,
      dest,
      promotion
    }
  }

  pub fn get_source(&self) -> Square { self.source }
  pub fn get_dest(&self) -> Square { self.dest }
  pub fn get_promotion(&self) -> Option<Piece> { self.promotion }
}
