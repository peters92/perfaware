use std::time::{Duration, Instant};
use std::thread::sleep;
use std::io::stdin;

fn main() {
    let mut duration = String::new();
    println!("Input timer duration in ns:");
    stdin().read_line(&mut duration).expect("couldn't read input");
    let duration: u64 = duration.trim().parse().unwrap();

    let now = Instant::now();
    let os_start = now.elapsed().as_nanos();
    
    sleep(Duration::from_nanos(duration));
    
    let os_end = now.elapsed().as_nanos();
    let os_elapsed = os_end - os_start;

    println!("OS timer: {:?} -> {:?} = {:?}", os_start, os_end, os_elapsed);
    println!("Ticks elapsed per duration: {:?}", os_elapsed / duration as u128);
}

    