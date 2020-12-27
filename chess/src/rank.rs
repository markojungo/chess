use crate::error::Error;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Hash)]
pub enum Rank {
  First,
  Second,
  Third,
  Fourth,
  Fifth,
  Sixth,
  Seventh,
  Eighth
}

pub const NUM_RANKS: u8 = 8;

impl FromStr for Rank {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.len() != 1 { return Err(Error::InvalidRank) }

    let ind = s.parse::<u8>();
    match ind {
      Ok(i) => Ok(Rank::from_index(i)),
      _     => Err(Error::InvalidRank)
    }
  }
}

impl Rank {
  pub fn from_index(i: u8) -> Rank {
    let i = i % 8;
    match i {
      0 => Rank::First,
      1 => Rank::Second,
      2 => Rank::Third,
      3 => Rank::Fourth,
      4 => Rank::Fifth,
      5 => Rank::Sixth,
      6 => Rank::Seventh,
      7 => Rank::Eighth,
      _ => panic!("i should be between 0 and 7!")
    }
  }

  pub fn to_index(&self) -> u8 {
    *self as u8
  }

  pub fn up(&self) -> Option<Rank> {
    if *self == Rank::Eighth { return None }

    Some(Rank::from_index(self.to_index() + 1))
  }

  pub fn down(&self) -> Option<Rank> {
    if *self == Rank::First { return None } 
    
    Some(Rank::from_index(self.to_index() - 1))
  }
}

#[cfg(test)]
mod tests {
  use crate::rank::Rank;
  use std::str::FromStr;

  #[test]
  fn t_from_index() {
      assert_eq!(Rank::First, Rank::from_index(0));
      assert_eq!(Rank::Second, Rank::from_index(1));
      assert_eq!(Rank::First, Rank::from_index(8));
  }

  #[test]
  fn t_to_index() {
    assert_eq!(0, Rank::First.to_index());
    assert_eq!(7, Rank::Eighth.to_index());
  }

  #[test]
  fn t_up() {
    assert_eq!(Some(Rank::Second), Rank::First.up());
    assert_eq!(None, Rank::Eighth.up());
  }

  #[test]
  fn t_down() {
    assert_eq!(None, Rank::First.down());
    assert_eq!(Some(Rank::First), Rank::Second.down());
  }

  #[test]
  fn t_from_str() {
    assert_eq!(Rank::First, Rank::from_str("0").unwrap());
  }
}