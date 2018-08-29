extern crate libc;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use libc::{sysctlbyname, size_t};
use std::mem;
use std::ptr;
use std::ffi::{CString};

mod args;
pub use args::*;

mod swap;
pub use swap::*;

mod mem_stats;
pub use mem_stats::*;

mod zfs_arc_stats;
pub use zfs_arc_stats::*;

pub fn get_pagesize() -> u32 {
  sysctl("hw.pagesize")
}

pub fn sysctl<T>(name: &str) -> T {
  let mut len: size_t = mem::size_of::<T>();
  unsafe {
    let mut v: T = mem::uninitialized();
    let cname = CString::new(name).unwrap();
    let res = sysctlbyname(cname.as_ptr(), mem::transmute(&mut v), &mut len, ptr::null(), 0);
    assert!(res == 0, format!("sysctlbyname for {} failed", name));
    v
  }
}
