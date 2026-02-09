use crate::file_reader;

pub fn solve_a() {
    let lines = file_reader::read_file_lines("input_data/day14/day14.in");

    let mut total_weight = 0;

    for x in 0..lines[0].len() {
        let mut last_y_can_fall = 0;

        for y in 0..lines.len() {
            if lines[y].chars().nth(x).unwrap() == 'O' {
                total_weight += lines.len() - last_y_can_fall;
                last_y_can_fall += 1;
            } else if lines[y].chars().nth(x).unwrap() == '#' {
                last_y_can_fall = y + 1;
            }
        }
    }

    println!("{}", total_weight);
}

pub fn solve_b() {

}