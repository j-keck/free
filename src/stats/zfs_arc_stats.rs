use super::*;

#[derive(Debug)]
pub struct ZfsArcStats {
  pub total: u64,
  pub mfu: u64,
  pub mru: u64,
  pub anon: u64,
  pub hdr: u64,
  pub l2_hdr: u64,
  pub other: u64,
}

impl ZfsArcStats {
  pub fn used(&self) -> u64 {
    self.mfu + self.mru + self.anon + self.hdr + self.other
  }

  pub fn free(&self) -> u64 {
    self.total - self.used()
  }
}

pub fn get_zfs_arc_stats() -> Result<ZfsArcStats> {
  Ok(ZfsArcStats {
    total: sysctl("vfs.zfs.arc_max")?,
    mfu: sysctl("vfs.zfs.mfu_size")?,
    mru: sysctl("vfs.zfs.mru_size")?,
    anon: sysctl("vfs.zfs.anon_size")?,
    hdr: sysctl("kstat.zfs.misc.arcstats.hdr_size")?,
    l2_hdr: sysctl("kstat.zfs.misc.arcstats.l2_hdr_size")?,
    other: sysctl("kstat.zfs.misc.arcstats.other_size")?,
  })
}
