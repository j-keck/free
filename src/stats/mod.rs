use libc::{sysctlbyname, size_t, c_void};
use std::mem;
use std::ptr;
use std::ffi::{CString};


use ::Result;

mod swap_stats;
pub use stats::swap_stats::*;

mod mem_stats;
pub use stats::mem_stats::*;

mod zfs_arc_stats;
pub use stats::zfs_arc_stats::*;

#[derive(Debug)]
pub struct Stats {
  pub mem: MemStats,
  pub swap: SwapStats,
  pub zfs_arc: Option<ZfsArcStats>,
}

impl Stats {
  pub fn collect() -> Result<Stats> {
    // propagate errors
    let mem = get_mem_stats()?;

    // propagate errors
    let swap = get_swap_stats()?;

    // use None if a error occurs (not all systems have zfs)
    let zfs_arc = get_zfs_arc_stats().ok();

    Ok(Stats{ mem, swap, zfs_arc })
  }

  pub fn total(&self) -> u64 {
    self.mem.total + self.swap.total
  }

  pub fn used(&self) -> u64 {
    self.mem.used() + self.swap.used
  }

  pub fn available(&self) -> u64 {
    self.mem.available() + self.swap.free
  }
}



pub fn get_pagesize() -> u32 {
  sysctl("hw.pagesize").expect("sysctl 'hw.pagesize' not found!") as u32
}

pub fn sysctl(name: &str) -> Result<u64> {
  let mut len: size_t = mem::size_of::<u64>();
  unsafe {
    let mut v: u64 = mem::uninitialized();
    let cname = CString::new(name).unwrap();
    if sysctlbyname(cname.as_ptr(), &mut v as *mut u64 as *mut c_void, &mut len, ptr::null(), 0) == 0 {
      Ok(v)
    } else {
      Err(error!("sysctlbyname for {} failed", name))
    }
  }
}
