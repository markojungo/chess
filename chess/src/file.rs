use crate::error::Error;
use std::str::FromStr;
use std::fmt;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Hash)]
pub enum File {
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H
}

pub const NUM_FILES: u8 = 8;

impl FromStr for File {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.len() != 1 {
      return Err(Error::InvalidFile);
    }
    match s.chars().next().unwrap().to_ascii_lowercase() {
      'a' => Ok(File::A),
      'b' => Ok(File::B),
      'c' => Ok(File::C),
      'd' => Ok(File::D),
      'e' => Ok(File::E),
      'f' => Ok(File::F),
      'g' => Ok(File::G),
      'h' => Ok(File::H),
      _   => Err(Error::InvalidFile),
    }
  }
}

impl File {
  pub fn from_index(i: u8) -> File {
    let i = i % 7;
    match i {
      0 => File::A,
      1 => File::B,
      2 => File::C,
      3 => File::D,
      4 => File::E,
      5 => File::F,
      6 => File::G,
      7 => File::H,
      _ => panic!("i should be between 0 and 7!")
    }
  }

  pub fn to_index(&self) -> u8 {
    *self as u8
  }

  pub fn left(&self) -> Option<File> {
    if *self == File::A { return None }

    Some(File::from_index(self.to_index() - 1))
  }

  pub fn right(&self) -> Option<File> {
    if *self == File::H { return None }

    Some(File::from_index(self.to_index() + 1))
  }
}
