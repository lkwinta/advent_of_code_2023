use std::cmp::{max, min};
use crate::file_reader;

pub fn solve(){
    let map = file_reader::read_file_lines("input_data/day11/day11.in")
        .iter()
        .map(|str| str.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut clear_lines: Vec<usize> = Vec::new();
    let width = map[0].len();
    let height = map.len();

    for y in 0..height{
        if !map[y].contains(&'#') {
            clear_lines.push(y);
        }
    }

    let mut clear_columns: Vec<usize> = Vec::new();
    'x_loop: for x in 0..width{
        for y in 0..height {
            if map[y][x] == '#'{
                continue 'x_loop
            }
        }

        clear_columns.push(x);
    }

    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            if map[y][x] == '#' {
                galaxies.push((y, x));
            }
        }
    }

    let mut sum_a = 0;
    let mut sum_b = 0;
    for i in 0..galaxies.len(){
        for j in i + 1..galaxies.len(){
            let dist_y = galaxies[i].0.abs_diff(galaxies[j].0);
            let dist_x = galaxies[i].1.abs_diff(galaxies[j].1);

            sum_a += dist_y + dist_x;
            sum_b += dist_y + dist_x;

            for line in &clear_lines {
                if *line > min(galaxies[i].0, galaxies[j].0) &&  *line < max(galaxies[i].0, galaxies[j].0){
                    sum_a += 2 - 1;
                    sum_b += 1000000 - 1;
                }
            }

            for column in &clear_columns {
                if *column > min(galaxies[i].1, galaxies[j].1) &&  *column < max(galaxies[i].1, galaxies[j].1){
                    sum_a += 2 - 1;
                    sum_b += 1000000 - 1;
                }
            }
        }
    }

    println!("{}", sum_a);
    println!("{}", sum_b);
}