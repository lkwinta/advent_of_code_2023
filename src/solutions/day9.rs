use crate::file_reader;

fn calc_layers(mut current: Vec<i32>) -> Vec<Vec<i32>> {
    let mut layers: Vec<Vec<i32>> = Vec::new();

    while !current.iter().all(|num| num == &0) {
        layers.push(current.clone());
        current = Vec::new();

        for i in 1..layers[layers.len() - 1].len() {
            current.push(layers[layers.len() - 1][i] - layers[layers.len() - 1][i - 1]);
        }
    }

    current.insert(0, 0);
    layers.push(current);

    layers
}

pub fn solve_a() {
    let lines = file_reader::read_file_lines("input_data/day9/day9.in");
    let mut sum = 0;
    for line in lines {

        let current = line.split(' ').map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let mut layers = calc_layers(current);

        for i in (0..layers.len() - 1).rev() {
            let last_prev = *layers[i+1].last().unwrap();
            let last_curr = *layers[i].last().unwrap();
            layers[i].push(last_prev + last_curr);
        }

        sum += layers[0].last().unwrap();
    }

    println!("{}", sum);
}

pub fn solve_b() {
    let lines = file_reader::read_file_lines("input_data/day9/day9.in");
    let mut sum = 0;
    for line in lines {

        let current = line.split(' ').map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let mut layers = calc_layers(current);

        for i in (0..layers.len() - 1).rev() {
            let first_prev = *layers[i+1].first().unwrap();
            let first_curr = *layers[i].first().unwrap();

            layers[i].insert(0, first_curr - first_prev);
        }

        sum += layers[0][0];
    }

    println!("{}", sum);
}