use std::cmp::max;
use ::Unit;
use ::Stats;
use ::Args;

pub fn print_report(stats: &Stats, args: &Args) {
  let unit = &args.unit();

  macro_rules! max_len {
    ($($x:expr),*) => {
      {
        let mut w = 0;
        $(w = max(w, $x.len());)*
        w
      }
    };
  }


  let fmt3 = |mem: u64, swap: u64, total: u64| {
    let (mem_str, swap_str, total_str) = (fmt(mem, unit), fmt(swap, unit), fmt(total, unit));
    (mem_str, swap_str, total_str)
  };

  let println_stat = |name: &str, total: &str, total_w: usize, used: &str, used_w: usize, free: &str, free_w: usize| {
      println!("{0:<10}      {1:>2$}      {3:>4$}      {5:>6$}", name, total, total_w, used, used_w, free, free_w);
  };

  let (mem_total, swap_total, total_total) = fmt3(stats.mem.total(), stats.swap.total, stats.total());
  let (mem_used, swap_used, total_used) = fmt3(stats.mem.used(), stats.swap.used, stats.used());
  let (mem_free, swap_free, total_free) = fmt3(stats.mem.available(), stats.swap.free, stats.available());

  let (arc_total, arc_used, arc_free) = stats.zfs_arc.as_ref().map(|arc|
    (fmt(arc.total, unit), fmt(arc.used(), unit), fmt(arc.free(), unit))
  ).unwrap_or(("".to_string(), "".to_string(), "".to_string()));

  let total_w = max_len!("total", mem_total, swap_total, total_total, arc_total);
  let used_w = max_len!("used", mem_used, swap_used, total_used, arc_used);
  let free_w = max_len!("free", mem_free, swap_free, total_free, arc_free);


  println_stat("", "total", total_w, "used", used_w, "free", free_w);
  println_stat("Mem:", &mem_total, total_w, &mem_used, used_w, &mem_free, free_w);
  println_stat("Swap:", &swap_total, total_w, &swap_used, used_w, &swap_free, free_w);
  if args.total || args.all {
    println_stat("Total:", &total_total, total_w, &total_used, used_w, &total_free, free_w);
  }

  if args.zfs || args.all {
    println_stat("ARC:", &arc_total, total_w, &arc_used, used_w, &arc_free, free_w);
  }

  if args.detailed || args.all {
    println!();
    stats.zfs_arc.as_ref().map(|arc| {
        println!("ARC: {} Total, {} MFU, {} MRU, {} Anon, {} Header, {} Other",
                 fmt(arc.total, unit), fmt(arc.mfu, unit),
                 fmt(arc.mru, unit), fmt(arc.anon, unit),
                 fmt(arc.hdr + arc.l2_hdr, unit),
                 fmt(arc.other, unit));
    });
    println!("Mem: {} Active, {} Inact, {} Laundry, {} Wired, {} Cache, {} Free",
             fmt(stats.mem.active, unit), fmt(stats.mem.inactive, unit),
             fmt(stats.mem.laundry, unit), fmt(stats.mem.wire, unit),
             fmt(stats.mem.cache, unit), fmt(stats.mem.free, unit));
  }
}




fn fmt(v: u64, unit: &Unit) -> String {
    use Unit::*;
    match unit {
        H => {
            let units = vec![Pi, Ti, Gi, Mi, Ki, B];
            let (v, u) = units
                .iter()
                .map(|u| convert(v, u)).find(|(v, _)| v > &1_f64)
                .unwrap_or((v as f64, B));
            format!("{:.1}{:?}", v, u)
        },
        _       => {
            let (v, _) = convert(v, unit);
            format!("{:.0}", v)
        },
    }
}

fn convert(v: u64, unit: &Unit) -> (f64, Unit) {
    let v = v as f64;

    use Unit::*;
    match unit {
        B  => (v, B),
        K  => (v / 1000_f64, K),
        Ki => (v / 10240_f64, Ki),
        M  => (v / 1_000_000_f64, M),
        Mi => (v / 1_048_576_f64, Mi),
        G  => (v / 1_000_000_000_f64, G),
        Gi => (v / 1_073_741_824_f64, Gi),
        T  => (v / 1_000_000_000_000_f64, T),
        Ti => (v / 1_099_511_627_776_f64, Ti),
        P  => (v / 1_000_000_000_000_000_f64, P),
        Pi => (v / 1_125_899_906_842_624_f64, Pi),
        H  => panic!("invalid unit"),
    }
}
