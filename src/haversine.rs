pub mod math {
    const EARTH_RADIUS: f64 = 6372.8;

    pub fn square(a: f64) -> f64 {
        a * a
    }

    pub fn radian_from_degrees(degree: f64) -> f64 {
        0.01745329251994329577f64 * degree
    }

    pub fn reference_haversine(x0: f64, y0: f64, x1: f64, y1: f64, earth_radius: f64) -> f64 {
        let lat_1: f64 = y0;
        let lat_2: f64 = y1;
        let lon_1: f64 = x0;
        let lon_2: f64 = x1;

        let delta_lat = radian_from_degrees(lat_2 - lat_1);
        let delta_lon = radian_from_degrees(lon_2 - lon_1);

        let lat_1 = radian_from_degrees(lat_1);
        let lat_2 = radian_from_degrees(lat_2);

        let a = square(f64::sin(delta_lat / 2.0))
            + f64::cos(lat_1) * f64::cos(lat_2) * square(f64::sin(delta_lon / 2.0));

        let c = 2.0 * f64::asin(f64::sqrt(a));

        earth_radius * c
    }

    pub fn vector_reference_haversine(pair_vector: &Vec<[f64; 4]>) -> f64 {
        let mut result: f64 = 0.0;

        for array in pair_vector {
            result += reference_haversine(array[0], array[1], array[2], array[3], EARTH_RADIUS)
        }
        result
    }
}
