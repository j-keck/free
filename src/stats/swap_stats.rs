use libc::{c_void, c_char, O_RDONLY};
use std::ffi::{CString};
use std::ptr;
use std::mem;

use ::Result;
use super::{get_pagesize, sysctl};

#[derive(Debug)]
pub struct SwapStats {
    pub total: u64,
    pub used:  u64,
    pub free:  u64,
}


pub fn get_swap_stats() -> Result<SwapStats> {

    let ks = unsafe {
        let cstr_dev_null = CString::new("/dev/null").unwrap();
        let cstr_kvm_open = CString::new("kvm_open").unwrap();
        let kd = kvm_open(ptr::null_mut()
                          , cstr_dev_null.as_ptr()
                          , cstr_dev_null.as_ptr()
                          , O_RDONLY
                          , cstr_kvm_open.as_ptr());
        let mut ks: [kvm_swap; 1] = mem::uninitialized();
        kvm_getswapinfo(kd, ks.as_mut_ptr() as *mut c_void, 1, 0);
        kvm_close(kd);
        ks
    };

    let total = sysctl("vm.swap_total")?;

    let pagesize = get_pagesize();
    let used = u64::from(ks[0].used * pagesize);

    Ok(SwapStats{ used, total, free: total - used })
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
