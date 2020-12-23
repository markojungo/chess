mod board;
pub use crate::board::*;

mod piece;
pub use crate::piece::*;

#[cfg(test)]
mod tests {
    #[test]
    fn t1() {
        assert_eq!(2,2);
    }
}
