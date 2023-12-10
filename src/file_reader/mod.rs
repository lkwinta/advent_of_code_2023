use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn read_file_lines(path: &str) -> Vec<String> {
    let input_file = File::open(path).unwrap();
    let input_reader = BufReader::new(input_file);

    return input_reader.lines().collect::<Result<_, _>>().unwrap();
}