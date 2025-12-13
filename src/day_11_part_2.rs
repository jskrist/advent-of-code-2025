use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Output {
    count: u64,
    waypoints: Vec<String>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Inputs {
    key: String,
    waypoints: Vec<String>,
}

fn one_hop(map: &HashMap<&str, Vec<&str>>, key: &str, mut waypoints: Vec<String>, cache: &mut HashMap<Inputs, Output>) -> Output {
    // take one hop at a time until we reach the output then return the number of times we reached the output
    let mut num_paths = 0;
    // check for a cached solution
    let input = Inputs{ key: key.to_string(), waypoints: waypoints.clone() };
    if let Some(cached) = cache.get(&input) {
        return cached.clone();
    }

    for &k in map.get(key).unwrap() {
        // add waypoint if it's a special node and not already in list
        if (k == "dac" || k == "fft") && !waypoints.iter().any(|w| w == k) {
            waypoints.push(k.to_string());
        }
        // if we've reached the "out" node, then check if we've visited the waypoints
        if k == "out" {
            if waypoints.len() == 2 {
                let out = Output { count: 1, waypoints: waypoints.clone() };
                cache.insert(Inputs{ key: key.to_string(), waypoints: waypoints.clone() }, out.clone());
                return out
            } else {
                let out = Output { count: 0, waypoints: waypoints.clone() };
                cache.insert(Inputs{ key: key.to_string(), waypoints: waypoints.clone() }, out.clone());
                return out
            }
        } else {
            // recurse to check the next hop in the network
            let out = one_hop(map, k, waypoints.clone(), cache);
            num_paths += out.count;
            waypoints = out.waypoints;
        }
        // after we come out of the depth first search recursion, remove this special node from the waypoints
        // if we are switching to a different branch
        if (k == "dac" || k == "fft") && waypoints.iter().any(|w| w == k) {
            waypoints.retain(|w| w != k);
        }
    }

    let out = Output { count: num_paths, waypoints: waypoints.clone() };
    cache.insert(Inputs{ key: key.to_string(), waypoints: waypoints.clone() }, out.clone());
    out
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

    let waypoints: Vec<String> = Vec::new();
    let mut cache: HashMap<Inputs, Output> = HashMap::new();
    let out = one_hop(&map, "svr", waypoints, &mut cache);
    println!("There are {} paths", out.count);
}
