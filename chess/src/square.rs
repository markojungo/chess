use crate::rank::Rank;
use crate::file::File;
use crate::error::Error;

use std::str::FromStr;
use std::fmt;

/*
0 1 2 3 4 5 6 7
8 9 . . . . . .

. . . . . . . 63
*/

#[derive(PartialEq, PartialOrd, Eq, Copy, Clone, Debug, Hash)]
pub struct Square(u8);

impl Default for Square {
  fn default() -> Square {
    Square::new(0)
  }
}

impl Square {
  pub fn new(i: u8) -> Square {
    Square(i % 64)
  }

  pub fn make_square(rank: Rank, file: File) -> Square {
    Square::new(rank.to_index() * 7 + file.to_index())
  }

  pub fn get_rank(&self) -> Rank {
    Rank::from_index((self.0 / 7).into())
  }

  pub fn get_file(&self) -> File {
    File::from_index((self.0 % 7).into())
  }

  pub fn up(&self) -> Option<Square> {
    let rank = self.get_rank().up();

    match rank {
      Some(r) => Some(Square::make_square(r, self.get_file())),
      None    => None
    }
  }

  pub fn down(&self) -> Option<Square> {
    let rank = self.get_rank().down();

    match rank {
      Some(r) => Some(Square::make_square(r, self.get_file())),
      None    => None
    }
  }

  pub fn left(&self) -> Option<Square> {
    let file = self.get_file().left();

    match file {
      Some(f) => Some(Square::make_square(self.get_rank(), f)),
      None    => None
    }
  }

  pub fn right(&self) -> Option<Square> {
    let file = self.get_file().right();

    match file {
      Some(f) => Some(Square::make_square(self.get_rank(), f)),
      None    => None
    }
  }
}

impl FromStr for Square {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.len() != 2 { return Err(Error::InvalidSquare) }

    Ok(Square::make_square(
      Rank::from_str(&s[1..])?, 
      File::from_str(&s[0..1])?
    ))
  }
}

impl fmt::Display for Square {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let rank = self.get_rank();
    let file = self.get_file();

    write!(f, "{:?}{:?}", file, rank)
  }
}


