use std::env;

mod day_1_part_1;
mod day_1_part_2;
mod day_2_part_1;
mod day_2_part_2;
mod day_3_part_1;
mod day_3_part_2;
mod day_4_part_1;
mod day_4_part_2;
mod day_5_part_1;
mod day_5_part_2;
mod day_6_part_1;
mod day_6_part_2;
mod day_7_part_1;
mod day_7_part_2;
mod day_8_part_1;
mod day_8_part_2;
mod day_9_part_1;
mod day_9_part_2;
mod day_11_part_1;
mod day_11_part_2;

fn run_day(day_number:u8, scenario: &String, input_root: &String) {
    match day_number {
        1 => {
            println!("Day {} Part 1:", day_number);
            day_1_part_1::main(scenario, input_root.to_string());
            println!("Day {} Part 2:", day_number);
            day_1_part_2::main(scenario, input_root.to_string());
        }
        2 => {
            println!("Day {} Part 1:", day_number);
            day_2_part_1::main(scenario, input_root.to_string());
            println!("Day {} Part 2:", day_number);
            day_2_part_2::main(scenario, input_root.to_string());
        }
        3 => {
            println!("Day {} Part 1:", day_number);
            day_3_part_1::main(scenario, input_root.to_string());
            println!("Day {} Part 2:", day_number);
            day_3_part_2::main(scenario, input_root.to_string());
        }
        4 => {
            println!("Day {} Part 1:", day_number);
            day_4_part_1::main(scenario, input_root.to_string());
            println!("Day {} Part 2:", day_number);
            day_4_part_2::main(scenario, input_root.to_string());
        }
        5 => {
            println!("Day {} Part 1:", day_number);
            day_5_part_1::main(scenario, input_root.to_string());
            println!("Day {} Part 2:", day_number);
            day_5_part_2::main(scenario, input_root.to_string());
        }
        6 => {
            println!("Day {} Part 1:", day_number);
            day_6_part_1::main(scenario, input_root.to_string());
            println!("Day {} Part 2:", day_number);
            day_6_part_2::main(scenario, input_root.to_string());
        }
        7 => {
            println!("Day {} Part 1:", day_number);
            day_7_part_1::main(scenario, input_root.to_string());
            println!("Day {} Part 2:", day_number);
            day_7_part_2::main(scenario, input_root.to_string());
        }
        8 => {
            println!("Day {} Part 1:", day_number);
            day_8_part_1::main(scenario, input_root.to_string());
            println!("Day {} Part 2:", day_number);
            day_8_part_2::main(scenario, input_root.to_string());
        }
        9 => {
            println!("Day {} Part 1:", day_number);
            day_9_part_1::main(scenario, input_root.to_string());
            println!("Day {} Part 2:", day_number);
            day_9_part_2::main(scenario, input_root.to_string());
        }
        11 => {
            println!("Day {} Part 1:", day_number);
            day_11_part_1::main(scenario, input_root.to_string());
            println!("Day {} Part 2:", day_number);
            day_11_part_2::main(scenario, input_root.to_string());
        }
        _ => {}
    }
}

fn main() {
    // Inputs:
    //  day_number      - a number from 0-12 indicating which day's solutions to run
    //                    If 0 is given, all solutions are run.
    //  scenario        - either `test_1` or `input_1` indicating which input file to use
    //  input_root      - Optional; a root directory to look in for the input files; Default is `inputs`
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 2);
    let day_number = args[1].parse::<u8>().unwrap_or_default();
    let scenario = &args[2];
    let input_root: String;
    if args.len() > 3 {
        input_root = args[3].to_string();
    }
    else {
        input_root = "inputs".to_string();
    }
    match day_number {
        0 => {
            for day in 1..=12 {
                run_day(day, scenario, &input_root);
            }
        }
        1..=12 => {
            run_day(day_number, scenario, &input_root);
        }
        _ => {}
    }
}