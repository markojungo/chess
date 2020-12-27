use snafu::Snafu;

#[derive(Clone, Debug, Snafu)]
pub enum Error {
  #[snafu(display("Invalid rank."))]
  InvalidRank,
  #[snafu(display("Invalid file."))]
  InvalidFile,
  #[snafu(display("Invalid square."))]
  InvalidSquare,
}

