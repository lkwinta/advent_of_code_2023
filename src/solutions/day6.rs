use std::iter::zip;
use crate::file_reader;
use std::cmp::{min, max};

fn parse_line_to_vec(str:  &str) -> Vec<f64>{
    return str.split(": ")
        .nth(1).unwrap()
        .split(" ")
        .filter(|&x| !x.is_empty())
        .map(|x| x.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();
}

fn parse_line_to_f64(str: &str) -> f64 {
    return str
        .split(": ")
        .nth(1).unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<f64>().unwrap();
}

fn solve_quadratic(a: f64, b: f64, c: f64) -> (i64, i64) {
    let delta = f64::sqrt(f64::powf(b, 2.0) - 4.0*a*c);
    let t1 = (-b - delta)/(2.0*a);
    let t2 = (-b + delta)/(2.0*a);

    let t1_int = (f64::floor(t1) + 1.0) as i64;
    let t2_int = (f64::ceil(t2)) as i64;

    (min(t1_int, t2_int), max(t1_int, t2_int))
}

pub fn solve_a() {
    let mut mul = 1;

    let lines = file_reader::read_file_lines("input_data/day6/day6.in");
    let times = parse_line_to_vec(&lines[0]);
    let record_distances = parse_line_to_vec(&lines[1]);

    for (tc, d) in zip(times, record_distances){
        let (lower_bound, upper_bound) = solve_quadratic(-1.0, tc, -d);

        let ways = (lower_bound..upper_bound).count();

        mul *= ways;
    }

    println!("{}", mul);
}

pub fn solve_b() {
    let lines = file_reader::read_file_lines("input_data/day6/day6.in");
    let tc = parse_line_to_f64(&lines[0]);
    let d = parse_line_to_f64(&lines[1]);

    let (lower_bound, upper_bound) = solve_quadratic(-1.0, tc, -d);

    let ways = (lower_bound..upper_bound).count();
    println!("{}", ways);
}