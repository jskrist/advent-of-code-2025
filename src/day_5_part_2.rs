use std::fs;
use std::cmp::{min, max};

pub fn main(scenario: &String, input_root: String) {
    let input_file = input_root + "/day_5_" + scenario + ".txt";
    let input = fs::read_to_string(input_file).expect("Failed to read input file");

    // split the input into the ranges and ingredients list
    // line endings are tricky; remove any \r in the file to leave only \n
    let clean_input = input.replace("\r", "");
    let mut input_iter = clean_input.split("\n\n");
    let ranges = input_iter.next().unwrap();

    // created a structure for storing range data
    #[derive(Clone, Debug)]
    struct RangeData {
        start: u64,
        stop: u64,
        indices: Vec<u64>, // indices from original input that were used to create this range
    }
    // parse the ranges data
    let mut start_stops: Vec<RangeData> = Vec::new();    
    for (idx, range) in ranges.lines().enumerate() {
        // parse the strings into numeric values
        let start_stop:Vec<u64> = range.split("-").collect::<Vec<&str>>().iter()
                                       .map(|c| c.parse::<u64>().unwrap()).collect();
        start_stops.push(RangeData{start: start_stop[0], stop: start_stop[1], indices: vec![idx as u64]});
    }

    // combine ranges
    let mut ranges_to_check: Vec<RangeData> = start_stops.clone();
    let num_ranges = start_stops.len();
    let mut combined_ranges: Vec<RangeData> = Vec::with_capacity(num_ranges);
    let mut combined_range = ranges_to_check.pop().unwrap();
    // check each range to see if it can be combined with any of the remaining ranges
    while !ranges_to_check.is_empty() {
        let mut broke_out = false;
        for (idx, range) in ranges_to_check.iter().enumerate() {
            // if the ranges overlap, combine them, and remove the combined ranges from the
            // ranges_to_check, then break out of the loop and start again, since the newly
            // expanded range might overlap with previously checked ranges
            if (range.start <= combined_range.stop) & (range.stop >= combined_range.start) {
                combined_range.start = min(combined_range.start, range.start);
                combined_range.stop = max(combined_range.stop, range.stop);
                combined_range.indices = [range.indices.as_slice(), combined_range.indices.as_slice()].concat();
                ranges_to_check.remove(idx);
                broke_out = true;
                break;
            }
        }
        // if we didn't break out of the loop we didn't find any other ranges to combine with
        // so add the fully combined range to the list and move on to the next one to check
        if !broke_out {
            combined_ranges.push(combined_range.clone());
            combined_range = ranges_to_check.pop().unwrap();
        }
    }
    // make sure to add the final range to the list of combined ranges
    combined_ranges.push(combined_range.clone());

    // loop through the combined ranges and count up the number of IDs
    let mut num_ids = 0;
    for range in combined_ranges {
        num_ids += range.stop - range.start + 1;
    }

    println!("The number of fresh ids is {}", num_ids);
}
