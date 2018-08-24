use super::{get_pagesize, sysctl};

#[derive(Debug)]
pub struct MemStats {
  pub active: u64,
  pub inactive: u64,
  pub laundry: u64,
  pub wire: u64,
  pub cache: u64,
  pub free: u64,
}

impl MemStats {
  pub fn sum(&self) -> u64 {
    self.active + self.inactive + self.laundry +
    self.wire + self.cache + self.free
  }
}


pub fn get_mem_stats() -> MemStats {
  let pagesize = get_pagesize() as u64;
  MemStats {
    active: sysctl::<u64>("vm.stats.vm.v_active_count") * pagesize,
    inactive: sysctl::<u64>("vm.stats.vm.v_inactive_count") * pagesize,
    laundry: sysctl::<u64>("vm.stats.vm.v_laundry_count") * pagesize,
    wire: sysctl::<u64>("vm.stats.vm.v_wire_count") * pagesize,
    cache: sysctl("vfs.bufspace"),
    free: sysctl::<u64>("vm.stats.vm.v_free_count") * pagesize,
  }
}
