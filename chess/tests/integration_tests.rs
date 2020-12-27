use chess;

#[test]
fn default_board() {
  let b: chess::Board = chess::Board::default();

  println!("{}", b);

  assert_eq!(2, 2);
}