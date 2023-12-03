mod solutions;
pub mod file_reader;

pub fn main() {
    println!("Day1: ");
    solutions::day1::solve_a();
    solutions::day1::solve_b();
    println!("Day2: ");
    solutions::day2::solve_a();
    solutions::day2::solve_b();
    println!("Day3: ");
    solutions::day3::solve_a();
    solutions::day3::solve_b();
}
