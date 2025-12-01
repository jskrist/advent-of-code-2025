// read in the input file
use std::fs;

pub fn main() {
    let input_file = "day_1_puzzle_1_input.txt";
    let directions = fs::read_to_string(input_file).expect("Failed to read input file");
    let starting_point = 50;
    let mut current_value = starting_point;
    let mut zero_crossings = 0;

    for line in directions.lines() {
        let num: i32 = match line.chars().nth(0).unwrap() {
            'R' => line[1..].parse::<i32>().unwrap(),
            'L' => -line[1..].parse::<i32>().unwrap(),
            _ => 0,
        };
        current_value = (current_value + num).rem_euclid(100);
        if current_value == 0 {
            zero_crossings += 1;
        }
    }

    println!("The password is {}", zero_crossings);
}
