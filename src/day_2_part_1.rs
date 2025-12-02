use fancy_regex::Regex;
use std::fs;

pub fn main(scenario: &String, input_root: String) {
    let input_file = input_root + "/day_2_" + scenario + ".txt";
    let input = fs::read_to_string(input_file).expect("Failed to read input file");

    let mut invalid_ids: Vec<i64> = Vec::new();

    let start_stop_strs = input.split(",");
    // using a regex backreference here is really pretty slow, but it works
    let re = Regex::new(r"^(\d+)\1$").unwrap();

    for start_stop in start_stop_strs {
        let start_stop_pair: Vec<&str> = start_stop.split("-").collect();
        let start: i64 = start_stop_pair[0].parse().unwrap();
        let stop: i64 = start_stop_pair[1].parse().unwrap();
        for ind in start..=stop {
            let num_str = ind.to_string();
            if re.is_match(num_str.as_str()).expect("Regex failed") {
                invalid_ids.push(ind);
            }
        }
    }
    println!("The password is {}", invalid_ids.iter().sum::<i64>());
}
