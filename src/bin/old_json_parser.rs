use haversine::utils::math;
use std::time::Instant;
use std::fs::File;
use std::error::Error;
use std::io::{BufReader, Read};

fn main() -> Result<(), Box<dyn Error>> {
    // JSON PARSER MVP
    /*
    Given an input file, do the following:
    - load into String
    - given the span within [], parse line by line
    - Get values after x0, y0, x1, y1
     */

    // Read JSON file into string

    let start = Instant::now();
    let file = File::open("haversine_10000k.json").unwrap();
    let mut reader = BufReader::new(file);
    let mut json_string = String::new();
    reader.read_to_string(&mut json_string)?;
    let json_size = json_string.len();
    let end_read = start.elapsed().as_nanos();
    let end_conversion_to_vector = start.elapsed().as_nanos();
    let mut vector_pairs: Vec<[f64; 4]> = Vec::new();

    for line in json_string.lines() {
        // Clean up braces around line
        let line = line.replace("{", "");
        let line = line.replace("}", "");

        if line == "" {
            continue;
        }
        let mut pairs: Vec<f64> = Vec::new();
        // Split line and check contents
        for substring in line.splitn(5, ':') {
            for sub_substring in substring.splitn(2, ',') {
                let sub_substring = sub_substring.trim();

                let value: f64 = match sub_substring.parse() {
                    Ok(value) => value,
                    _ => continue,
                };
                pairs.push(value);
            }
        }
        if pairs.len() > 0 {
            vector_pairs.push([pairs[0], pairs[1], pairs[2], pairs[3]]);
        }
    }
    let end_parse = start.elapsed().as_nanos();
    println!(
        "Size of JSON: {:.2}MB
        \rSum of haversine distances: {:?}",
        json_size / 1000_000,
        math::vector_reference_haversine(&vector_pairs)
    );
    let end_haversine_calculation = start.elapsed().as_nanos();

    let ratio = 1e9 as f64;
    let total_time = (end_haversine_calculation as f64) / ratio;
    let haversine_time = ((end_haversine_calculation - end_parse) as f64) / ratio;
    let parse_time = ((end_parse - end_conversion_to_vector) as f64) / ratio;
    let conversion_time = ((end_conversion_to_vector - end_read) as f64) / ratio;
    let read_time = (end_read as f64) / ratio;
    println!(
        "Total time:      {:.2}s  ({:.2}%)
        \rRead time:       {:.2}s ({:.2}%)
        \rConversion time: {:.2}s ({:.2}%)
        \rParse time:      {:.2}s ({:.2}%)
        \rHaversine:       {:.2}s ({:.2}%)",
        total_time,
        100.0 * (total_time / total_time),
        read_time,
        100.0 * (read_time / total_time),
        conversion_time,
        100.0 * (conversion_time / total_time),
        parse_time,
        100.0 * (parse_time / total_time),
        haversine_time,
        100.0 * (haversine_time / total_time)
    );
    Ok(())
}
