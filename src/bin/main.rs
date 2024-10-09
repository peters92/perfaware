use rand::Rng;
use std::fs;

// Custom utils
use haversine::utils::math;

fn main() {
    // println!("Enter number of pairs for the generated data:");
    // let mut number_of_points = String::new();
    // stdin()
    //     .read_line(&mut number_of_points)
    //     .expect("Couldn't read input.");
    // let number_of_points: usize = number_of_points.trim().parse().unwrap();
    let number_of_points: usize = 10;

    fn generate_haversine(number_of_points: usize) -> Vec<[f64; 4]> {
        let mut rng = rand::thread_rng();
        let mut points: Vec<[f64; 4]> = Vec::with_capacity(number_of_points);

        for _ in 0..number_of_points {
            let x0: f64 = rng.gen_range(0.0..180.0);
            let y0: f64 = rng.gen_range(-90.0..90.0);
            let x1: f64 = rng.gen_range(0.0..180.0);
            let y1: f64 = rng.gen_range(-90.0..90.0);

            points.push([x0, y0, x1, y1]);
        }
        points
    }

    fn write_json(points: Vec<[f64; 4]>) -> std::io::Result<()> {
        // Construct JSON
        let mut json = String::from("{\n  \"pairs\": [");
        let number_of_points = points.len();
        for points in points {
            json.push_str(&format!(
                "\n    {{\"x0\": {}, \"y0\": {}, \"x1\": {}, \"y1\": {}}},",
                points[0], points[1], points[2], points[3]
            ))
        }
        _ = json.pop();
        json.push_str("\n    ]\n}");
        fs::write(
            format!("haversine_{}k.json", (number_of_points as f64) / 1000.0),
            json,
        )
    }

    fn write_ref_result(reference_sum: f64, number_of_points: usize)
        -> std::io::Result<()> {
        let output = String::from(format!("{}", reference_sum));

        fs::write(
            format!("reference_sum_{}k.txt", (number_of_points as f64) / 1000.0),
            output,
        )
    }

    // Test writing
    let points = generate_haversine(number_of_points);
    let reference_sum = math::vector_reference_haversine(&points);

    // Print information
    println!("Pair count: {}", number_of_points);
    println!("Expected sum: {}", reference_sum);

    _ = write_ref_result(reference_sum, number_of_points);
    _ = write_json(points);
}
