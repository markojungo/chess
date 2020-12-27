mod board;
pub use crate::board::*;

mod piece;
pub use crate::piece::*;

mod chess_move;
pub use crate::chess_move::*;

mod square;
pub use crate::square::*;

mod error;
pub use crate::error::*;

mod rank;
pub use crate::rank::*;

mod file;
pub use crate::file::*;

mod game;
pub use crate::game::*;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn t1() {
//         assert_eq!(2,2);
//     }
// }
