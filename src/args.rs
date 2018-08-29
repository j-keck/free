use ::Unit;

#[derive(StructOpt, Debug)]
pub struct Args {

  /// show human-readable output
  #[structopt(short = "h", long = "human", group = "unit")]
  pub human: bool,

  /// show output in bytes
  #[structopt(short = "b", long = "bytes", group = "unit")]
  pub bytes: bool,

  /// show output in kilobytes
  #[structopt(long = "kilo", group = "unit")]
  pub kilo: bool,

  /// show output in megabytes
  #[structopt(long = "mega", group = "unit")]
  pub mega: bool,

  /// show output in gigabytes
  #[structopt(long = "giga", group = "unit")]
  pub giga: bool,

  /// show output in terabytes
  #[structopt(long = "tera", group = "unit")]
  pub tera: bool,

  /// show output in petabytes
  #[structopt(long = "peta", group = "unit")]
  pub peta: bool,

  /// show output in kibibytes
  #[structopt(short = "k", long = "kibi", group = "unit")]
  pub kibi: bool,

  /// show output in mebibytes
  #[structopt(short = "m", long = "mebi", group = "unit")]
  pub mebi: bool,

  /// show output in gibibytes
  #[structopt(short = "g", long = "gibi", group = "unit")]
  pub gibi: bool,

  /// show output in tebibytes
  #[structopt(long = "tebi", group = "unit")]
  pub tebi: bool,

  /// show output in pebibytes
  #[structopt(long = "pebi", group = "unit")]
  pub pebi: bool,

  /// show total for RAM + ZFS ARC + SWAP
  #[structopt(short = "t", long = "total")]
  pub total: bool,

  /// show zfs arc
  #[structopt(short = "z", long = "zfs")]
  pub zfs: bool,

  /// show detailed memory stats
  #[structopt(short = "d", long = "detail")]
  pub detailed: bool,

  /// show all stats (-dz)
  #[structopt(short = "a", long = "all")]
  pub all: bool,

  /// print version
  #[structopt(short = "V", long = "version")]
  pub print_version: bool,
}

impl Args {

  pub fn unit(&self) -> Unit {
    use Unit::*;

    let mappings = vec![
       (self.bytes, B)
      ,(self.kilo,  K)
      ,(self.kibi,  Ki)
      ,(self.mega,  M)
      ,(self.mebi,  Mi)
      ,(self.giga,  G)
      ,(self.gibi,  Gi)
      ,(self.tera,  T)
      ,(self.tebi,  Ti)
      ,(self.peta,  P)
      ,(self.pebi,  Pi)
      ,(self.human, H)
    ];

    mappings.into_iter().find(|t| t.0).map(|t| t.1).unwrap_or(K)
  }
}
