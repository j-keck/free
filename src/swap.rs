use libc::{c_void, c_char, O_RDONLY};
use std::ffi::{CString};
use std::ptr;
use std::mem;

use super::{get_pagesize};

#[derive(Debug)]
pub struct SwapStats {
    pub used:  u64,
    pub total: u64,
    pub free:  u64,
}


pub fn get_swap_stats() -> SwapStats {

    let ks = unsafe {
        let kd = kvm_open(ptr::null_mut()
                          , CString::new("/dev/null").unwrap().as_ptr()
                          , CString::new("/dev/null").unwrap().as_ptr()
                          , O_RDONLY
                          , CString::new("kvm_open").unwrap().as_ptr());
        let mut ks: [kvm_swap; 1] = mem::uninitialized();
        kvm_getswapinfo(kd, ks.as_mut_ptr() as *mut c_void, 1, 0);
        kvm_close(kd);
        ks
    };


    let pagesize = get_pagesize();
    let used = (ks[0].used * pagesize) as u64;
    let total = (ks[0].total * pagesize) as u64;

    SwapStats{ used, total, free: total - used }
}



// https://github.com/freebsd/freebsd/blob/5d20d7ee042b4253912639b36c1e2417d86b90c1/lib/libkvm/kvm.h#L75
#[derive(Debug)]
#[repr(C)]
struct kvm_swap {
    devname:    [c_char; 32],
    used:       u32,
    total:      u32,
    flags:      i32,
    reserved1:  u32,
    reserved2:  u32,
}


#[link(name = "kvm")]
extern "C" {
    fn kvm_open(kvm_t: *mut c_void, uf: *const c_char, mf: *const c_char, flag: i32, errout: *const c_char) -> *mut c_void;
    fn kvm_getswapinfo(kvm_t: *mut c_void, swap_ary: *mut c_void, swap_max: i32, flags: i32) -> i32;
    fn kvm_close(kvm_t: *mut c_void) -> i32;
}
