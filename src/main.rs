extern crate free;
extern crate structopt;

use free::*;
use structopt::StructOpt;

fn main() {
    let args = Args::from_args();
    let stats = Stats::collect();
    print_report(&stats.unwrap(), &args);
}
