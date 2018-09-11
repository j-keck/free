use super::*;

#[derive(Debug)]
pub struct MemStats {
  pub total: u64,
  pub active: u64,
  pub inactive: u64,
  pub laundry: u64,
  pub wire: u64,
  pub cache: u64,
  pub free: u64,
}

impl MemStats {

  pub fn used(&self) -> u64 {
    self.active + self.wire
  }

  pub fn available(&self) -> u64 {
    self.free + self.inactive + self.cache
  }
}


pub fn get_mem_stats() -> Result<MemStats> {
  let pagesize = u64::from(get_pagesize());
  Ok(MemStats {
    total: sysctl("vm.stats.vm.v_page_count")? * pagesize,
    active: sysctl("vm.stats.vm.v_active_count")? * pagesize,
    inactive: sysctl("vm.stats.vm.v_inactive_count")? * pagesize,
    laundry: sysctl("vm.stats.vm.v_laundry_count")? * pagesize,
    wire: sysctl("vm.stats.vm.v_wire_count")? * pagesize,
    cache: sysctl("vfs.bufspace")?,
    free: sysctl("vm.stats.vm.v_free_count")? * pagesize,
  })
}
