use std::{env, str::FromStr};

mod day_1_part_1;
mod day_1_part_2;
mod day_2_part_1;
mod day_2_part_2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_root: String;
    if args.len() > 2 {
        input_root = String::from_str(&args[2]).unwrap();
    }
    else {
        input_root = "inputs".to_string();
    }
    println!("Day 1 Part 1:");
    day_1_part_1::main(&args[1], input_root.clone());
    println!("Day 1 Part 2:");
    day_1_part_2::main(&args[1], input_root.clone());
    println!("Day 2 Part 1:");
    day_2_part_1::main(&args[1], input_root.clone());
    println!("Day 2 Part 2:");
    day_2_part_2::main(&args[1], input_root.clone());
}