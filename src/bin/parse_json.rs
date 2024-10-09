use std::{fs::File, io::Read};
use haversine::{parser, utils, profiler::profiler};


fn main() {
    // TODO: Experiment with buffered reader vs. one time read
    let mut profiler = profiler::Profiler::build();
    profiler.init_node();
    let mut file = File::open("haversine_10000k.json").expect("JSON file should exist");
    let mut json = String::new();
    file.read_to_string(&mut json).expect("File should be valid UTF-8.");

    profiler.log("file open and read to string", json.len());

    println!(
        "JSON information after reading:
        \rCapacity: {}MB
        \rLength: {}MB",
        json.capacity() / 1000_000,
        json.len() / 1000_000
    );

    let result = parser::json::parse(json);
    profiler.log("json parse", result.len() * 32); // 4 * f64 = 32 Bytes per vector element

    let calculated_sum = utils::math::vector_reference_haversine(&result);
    profiler.log("haversine calc", result.len() * 32); // same amount of data as json parse?

    println!("Calculated sum: {}", calculated_sum);
    println!("{}", profiler);
}