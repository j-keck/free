use super::*;

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
  pub fn total(&self) -> u64 {
    self.active + self.inactive + self.laundry +
    self.wire + self.cache + self.free
  }

  pub fn used(&self) -> u64 {
    self.active + self.wire
  }

  pub fn available(&self) -> u64 {
    self.free + self.inactive + self.cache
  }
}


pub fn get_mem_stats() -> Result<MemStats> {
  let pagesize = get_pagesize() as u64;
  Ok(MemStats {
    active: sysctl::<u64>("vm.stats.vm.v_active_count")? * pagesize,
    inactive: sysctl::<u64>("vm.stats.vm.v_inactive_count")? * pagesize,
    laundry: sysctl::<u64>("vm.stats.vm.v_laundry_count")? * pagesize,
    wire: sysctl::<u64>("vm.stats.vm.v_wire_count")? * pagesize,
    cache: sysctl("vfs.bufspace")?,
    free: sysctl::<u64>("vm.stats.vm.v_free_count")? * pagesize,
  })
}
