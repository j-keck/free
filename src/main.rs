extern crate free;
extern crate structopt;

use free::*;
use structopt::StructOpt;

fn main() {

    let args = Args::from_args();
    let unit = &args.unit();

    println!("         total         used         free");

    let mem_stats = get_mem_stats();
    let zfs_arc_stats = get_zfs_arc_stats();
    println!( "Mem:     {}        {}        {}"
               , fmt(mem_stats.sum(), unit)
               , fmt(mem_stats.active + mem_stats.wire, unit)
               , fmt(mem_stats.free + mem_stats.inactive + mem_stats.cache, unit));



    let swap_stats = get_swap_stats();
    println!("Swap:    {}      {}        {}"
             , fmt(swap_stats.total, unit)
             , fmt(swap_stats.used, unit)
             , fmt(swap_stats.free, unit));


    if args.total {
        println!("Total:   {}        {}        {}",
                 fmt(mem_stats.sum() + swap_stats.total, unit),
                 fmt(mem_stats.active + mem_stats.wire + swap_stats.used, unit),
                 fmt(mem_stats.free + mem_stats.inactive + mem_stats.cache + swap_stats.free, unit));
    }


    if args.zfs {
        println!("\nArc: {} Total, {} MFU, {} MRU, {} Anon, {} Header, {} Other",
                 fmt(zfs_arc_stats.total, unit), fmt(zfs_arc_stats.mfu, unit),
                 fmt(zfs_arc_stats.mru, unit), fmt(zfs_arc_stats.anon, unit),
                 fmt(zfs_arc_stats.hdr + zfs_arc_stats.l2_hdr, unit),
                 fmt(zfs_arc_stats.other, unit));
    }

    if args.detailed {
        println!("Mem: {} Active, {} Inact, {} Laundry, {} Wired, {} Cache, {} Free",
                 fmt(mem_stats.active, unit), fmt(mem_stats.inactive, unit),
                 fmt(mem_stats.laundry, unit), fmt(mem_stats.wire, unit),
                 fmt(mem_stats.cache, unit), fmt(mem_stats.free, unit));
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
