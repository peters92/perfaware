use perfaware::profiler::hp_profiler;
use std::arch::x86_64::__rdtscp;
use std::io::stdin;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::{env, thread};

fn main() {
    // let mut duration = String::new();
    // println!("Input timer duration in ns:");
    // stdin()
    //     .read_line(&mut duration)
    //     .expect("couldn't read input");
    // let duration: u64 = duration.trim().parse().unwrap();
    let duration = env::args().collect::<Vec<_>>()[1].trim().parse().unwrap();

    let now = Instant::now();
    let os_start = now.elapsed().as_nanos();

    sleep(Duration::from_nanos(duration));

    let os_end = now.elapsed().as_nanos();
    let os_elapsed = os_end - os_start;

    println!(
        "OS timer: {:?} -> {:?} = {:?}",
        os_start, os_end, os_elapsed
    );
    println!(
        "Ticks elapsed per duration: {:?}",
        os_elapsed / duration as u128
    );

    // Try RDTSC
    fn cycle_test() {
        let ts_1 = hp_profiler::read_tsc();
        thread::sleep(Duration::from_millis(1000));
        let ts_2 = hp_profiler::read_tsc();

        println!(
            "Cycles elapsed: {}
            \rRDTSC frequency: {}MHz",
            ts_2 - ts_1,
            (ts_2 - ts_1) / 1000_000
        )
    }

    match env::consts::OS {
        "windows" => {for _ in 0..10 {cycle_test()}},
        _ => {}
    }
}
