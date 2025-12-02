// read in the input file
use std::fs;

pub fn main() {
    // let input_file = "day_1_puzzle_1_test_1.txt";
    let input_file = "day_1_puzzle_1_input.txt";
    let directions = fs::read_to_string(input_file).expect("Failed to read input file");
    let starting_point = 50;
    let mut current_value = starting_point;

    let mut password = 0i32;
    for line in directions.lines() {
        let offset: i32 = match line.chars().nth(0).unwrap() {
            'R' => line[1..].parse::<i32>().unwrap(),
            'L' => -line[1..].parse::<i32>().unwrap(),
            _ => 0,
        };
        let next_value = current_value + offset;
        // Check for zero
        if next_value == 0 {
            password += 1;
        }
        else if current_value > 0 && next_value < 0 {
            password += 1;
        }
        // Check for wrap around
        password += next_value.abs() / 100i32;

        current_value = (next_value).rem_euclid(100);
    }

    println!("The password is {}", password);
}
