use std::fs;

pub fn main(scenario: &String, input_root: String) {
    let input_file = input_root + "/day_5_" + scenario + ".txt";
    let input = fs::read_to_string(input_file).expect("Failed to read input file");

    // split the input into the ranges and ingredients list
    // line endings are tricky; remove any \r in the file to leave only \n
    let clean_input = input.replace("\r", "");
    let mut input_iter = clean_input.split("\n\n");
    let ranges = input_iter.next().unwrap();
    let ids = input_iter.next().unwrap_or("A");

    // split up the ranges and recombine them
    let mut start_stops: Vec<Vec<u64>> = Vec::new();
    
    for range in ranges.lines() {
        let start_stop:Vec<u64> = range.split("-").collect::<Vec<&str>>().iter().map(|c| c.parse::<u64>().unwrap()).collect();
        start_stops.push(start_stop);
    }
    let mut fresh_count: u64 = 0;
    // check ids
    for id_str in ids.lines() {
        let id = id_str.parse::<u64>().unwrap();
        for start_stop in start_stops.iter() {
            if (id >= start_stop[0]) & (id <= start_stop[1]) {
                fresh_count += 1;
                break;
            }
        }
    }
    println!("The number of fresh ingredients is {}", fresh_count);
}
