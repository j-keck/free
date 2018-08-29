extern crate free;
extern crate structopt;

use free::*;
use structopt::StructOpt;

fn main() {
    let args = Args::from_args();

    if args.print_version {
      println!("{}: v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    } else {
      let stats = Stats::collect();
      print_report(&stats.unwrap(), &args);
    }
}
