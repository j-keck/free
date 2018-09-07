extern crate free;

use std::process::Command;




#[test]
fn validate_stats() {
  let stats = free::Stats::collect().unwrap();

  let pagesize = sysctl("hw.pagesize");

  let mem_total = sysctl("vm.stats.vm.v_page_count") * pagesize;
  assert_eq!(stats.mem.total, mem_total, "mem");

  let swap_total = sysctl("vm.swap_total");
  assert_eq!(stats.swap.total, swap_total, "swap");

}



fn sysctl(name: &str) -> u64 {
  let raw = Command::new("sysctl")
    .arg("-n")
    .arg(name)
    .output()
    .expect(&format!("Failed to execute 'sysctl -n {}", name))
    .stdout;

  // convert to string
  let mut str = String::from_utf8_lossy(&raw).to_string();

  // remove the newline sequence
  str.pop();

  // parse the string
  str.parse().expect(&format!("unable to convert '{}' to a number", str))
}
