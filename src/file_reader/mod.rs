use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn read_file_lines(path: &str) -> Vec<String> {
    let input_a_file = File::open(path).unwrap();
    let input_a_reader = BufReader::new(input_a_file);

    return input_a_reader.lines().collect::<Result<_, _>>().unwrap();
}