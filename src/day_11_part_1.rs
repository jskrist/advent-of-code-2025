use std::fs;
use std::collections::HashMap;

fn one_hop(map: &HashMap<&str, Vec<&str>>, key: &str) -> u32 {
    // take one hop at a time until we reach the output then return the number of times we reached the output
    let mut num_paths = 0;
    for k in map.get(key).unwrap() {
        if *k == "out" {
            return 1
        }
        else {
            num_paths += one_hop(map, k)
        }
    }
    num_paths
}

pub fn main(scenario: &String, input_root: String) {
    let input_file = input_root + "/day_11_" + scenario + ".txt";
    let input = fs::read_to_string(input_file).expect("Failed to read input file");

    // transform the input into a hashmap
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let key_vals: Vec<_> = line.split(":").map(|s|s.trim()).collect();
        let key = key_vals[0];
        map.insert(key, key_vals[1].split(" ").collect());
    };
    // println!("input map: {:?}", map);
    if !map.contains_key("you") {
        return
    }
    let num_paths = one_hop(&map, "you");
    println!("There are {} paths", num_paths);
}
