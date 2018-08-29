extern crate libc;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;


mod args;
pub use args::*;

#[macro_use] mod error;
pub use error::*;


mod stats;
pub use stats::*;


mod report;
pub use report::*;

pub type Result<T> = std::result::Result<T, error::Error>;


#[derive(Debug)]
pub enum Unit {
  B, K, Ki, M, Mi, G, Gi, T, Ti, P, Pi, H
}
