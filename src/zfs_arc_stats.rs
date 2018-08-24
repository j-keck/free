use super::{sysctl};

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


pub fn get_zfs_arc_stats() -> ZfsArcStats {
  ZfsArcStats {
    total: sysctl("kstat.zfs.misc.arcstats.size"),
    mfu: sysctl("vfs.zfs.mfu_size"),
    mru: sysctl("vfs.zfs.mru_size"),
    anon: sysctl("vfs.zfs.anon_size"),
    hdr: sysctl("kstat.zfs.misc.arcstats.hdr_size"),
    l2_hdr: sysctl("kstat.zfs.misc.arcstats.l2_hdr_size"),
    other: sysctl("kstat.zfs.misc.arcstats.other_size"),
  }
}
