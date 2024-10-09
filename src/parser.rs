/// JSON Parser module
///
/// This module contains all the logic needed for parsing a JSON file for the Haversine
/// distance problem.
pub mod json {
    use std::str::Chars;

    pub fn parse(json: String) -> Vec<[f64; 4]> {
        let json = remove_whitespace(json);
        // Start decoding
        let mut json_iterator = json.chars();

        // Find the first colon, that's the beginning of the json-array
        while let Some(c) = json_iterator.next() {
            if c == ':' {break}
        }

        let result = parse_array(&mut json_iterator);
        result
    }

    fn parse_array(json: &mut Chars<'_>) -> Vec<[f64; 4]> {
        let mut array = Vec::new();

        while let Some(c) = json.next() {
            if c == ']' {break}
            else {
                parse_object(json, &mut array)
            }
        }
        array
    }

    fn parse_object(json: &mut Chars<'_>, array: &mut Vec<[f64; 4]>) {
        let mut object = Vec::new();
        let _ = json.next(); // consume the opening '{'
        for _ in 0..4 {
            parse_string(json);
            object.push(parse_number(json));
        }

        array.push([object[0], object[1], object[2], object[3]]);
    }

    fn parse_string(json: &mut Chars<'_>) {
        let _ = json.next().unwrap(); // consume the opening '"'
        while let Some(c) = json.next() {
            if c != '"' {continue}
            else {break}
        }
    }

    fn parse_number(json: &mut Chars<'_>) -> f64 {
        let mut number = String::new();

        while let Some(c) = json.next() {
            if c == ':' {
                // do nothing
            } else if c == ',' || c == '}' { break }
            else {
                number.push(c);
            }
        }

        match number[..].parse::<f64>() {
            Ok(value) => value,
            Err(_) => {
                println!("Number: {number}");
                panic!();
            }
        }
    }
    fn remove_whitespace(json: String) -> String{
        // Remove all whitespace
        json.as_bytes()
            .into_iter()
            .filter(|x| !x.is_ascii_whitespace())
            .map(|x| *x as char)
            .collect()
    }

    // OLD
    pub fn substring_is_numeric(input: &str) -> bool {
        match input {
            "\"x0\"" => true,
            "\"y0\"" => true,
            "\"x1\"" => true,
            "\"y1\"" => true,
            _ => false
        }
    }
}