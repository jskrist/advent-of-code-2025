use std::cmp;
use std::fs;

pub fn main(scenario: &String, input_root: String) {
    let input_file = input_root + "/day_3_" + scenario + ".txt";
    let input = fs::read_to_string(input_file).expect("Failed to read input file");

    let mut total_joltage: u64 = 0;

    // loop through the input lines to find the max joltage possible
    for line in input.lines() {
        let mut chars_iter = line.chars().rev();
        // initialize the tens and ones digits based on the last two digits in the line
        let mut max_ones:u8 = chars_iter.next().unwrap().to_string().parse().unwrap();
        let mut max_tens:u8 = chars_iter.next().unwrap().to_string().parse().unwrap();
        let mut possible_max_ones:u8 = cmp::max(max_tens, max_ones);
        for c in chars_iter {
            let this_digit:u8 = c.to_string().parse().unwrap();
            if this_digit >= max_tens {
                max_tens = this_digit;
                max_ones = possible_max_ones;
            }
            if this_digit > possible_max_ones {
                possible_max_ones = this_digit;
            }
        }
        total_joltage += max_tens as u64 * 10 + max_ones as u64;
    }
    println!("The total joltage is {}", total_joltage);
}
