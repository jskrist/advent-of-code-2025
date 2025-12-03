use std::fs;

// simple structure to hold the maximum value and its index
struct MaxData {
    value: u8,
    ind: usize,
}

fn get_max_digit(all_digits: &Vec<u8>, start_ind: usize, stop_offset: usize) -> MaxData {
    // slice all the digits from the line into a subset of possible values for the next digit,
    // then iterate through the slice backwards, enumerating the values, so we can get the max
    // value along with its index.
    // 
    // returns the max digit found, along with its index relative to the start of the original vec
    let (max_index, max_value) = all_digits[start_ind..all_digits.len() - stop_offset]
            .iter().rev()
            .enumerate()
            .max_by_key(|&(_, &value)| value)
            .unwrap();
    return MaxData{value: *max_value, ind: all_digits.len() - stop_offset - max_index};
}

pub fn main(scenario: &String, input_root: String) {
    let input_file = input_root + "/day_3_" + scenario + ".txt";
    let input = fs::read_to_string(input_file).expect("Failed to read input file");

    let mut total_joltage: u64 = 0;
    const NUM_DIGITS: usize = 12;
    let mut local_max_joltage: u64 = 0;

    // loop through the input lines to find the max joltage possible
    for line in input.lines() {
        // convert the line of characters into a vector of numerical digits
        let all_digits: Vec<u8> = line.chars()
            .filter_map(|c| Some(c.to_string().parse::<u8>().expect("couldn't parse char")))
            .collect();
        // loop through each of the digits we are trying to maximize and serch for the largest
        // possible digit in the available range
        let mut start_ind = 0usize;
        for idx in 0..NUM_DIGITS {
            // call the function to find the largest digit in the range as well as where the first
            // instance is located
            let max_value = get_max_digit(&all_digits, start_ind, NUM_DIGITS - 1 - idx);
            // add this digit to the 
            local_max_joltage *= 10;
            local_max_joltage += max_value.value as u64;
            start_ind = max_value.ind;
        }
        total_joltage += local_max_joltage;
        // reset the local max joltage for the next loop
        local_max_joltage = 0;
    }
    println!("The total joltage is {}", total_joltage);
}
