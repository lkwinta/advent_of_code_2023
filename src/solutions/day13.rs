use crate::file_reader;

fn check_reflection_vertical(image: &[String], row_number: usize, tolerance: usize) -> bool {
    let mut diff = 0;
    for y in 0..row_number + 1 {
        let line_1 = y;
        let mid = row_number;
        let line_2 = 2*mid - line_1 + 1;

        if line_2 < image.len() {
            for x in 0..image[0].len() {
                if image[line_1].chars().nth(x).unwrap() != image[line_2].chars().nth(x).unwrap() {
                    diff += 1;
                    if diff > tolerance {
                        return false
                    }
                }
            }

        }
    }

    return diff == tolerance
}

fn check_reflection_horizontal(image: &[String], column_number: usize, tolerance: usize) -> bool {
    let mut diff = 0;
    for y in 0..column_number + 1 {
        let line_1 = y;
        let mid = column_number;
        let line_2 = 2*mid - line_1 + 1;

        if line_2 < image[0].len() {
            for y in 0..image.len() {
                if image[y].chars().nth(line_1).unwrap() != image[y].chars().nth(line_2).unwrap() {
                    diff += 1;
                    if diff > tolerance {
                        return false
                    }
                }
            }
        }
    }

    return diff == tolerance
}

pub fn solve_a() {
    let lines = file_reader::read_file_lines("input_data/day13/day13.in");
    let images = lines.split(|v| *v == String::from("")).collect::<Vec<_>>();

    let mut sum = 0;
    for image in images {
        for y in 0..image.len() - 1 {
            if check_reflection_vertical(image, y, 0) {
                sum += (y+1)*100;
                break;
            }
        }

        for x in 0..image[0].len() - 1 {
            if check_reflection_horizontal(image, x, 0) {
                sum += x + 1;
                break;
            }
        }
    }

    println!("{}", sum)
}

pub fn solve_b() {
    let lines = file_reader::read_file_lines("input_data/day13/day13.in");
    let images = lines.split(|v| *v == String::from("")).collect::<Vec<_>>();

    let mut sum = 0;
    for image in images {
        for y in 0..image.len() - 1 {
            if check_reflection_vertical(image, y, 1) {
                sum += (y+1)*100;
                break;
            }
        }

        for x in 0..image[0].len() - 1 {
            if check_reflection_horizontal(image, x, 1) {
                sum += x + 1;
                break;
            }
        }
    }

    println!("{}", sum)
}