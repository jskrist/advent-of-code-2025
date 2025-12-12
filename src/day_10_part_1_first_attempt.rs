use std::collections::HashMap;
use std::fs;
use std::iter::zip;

#[derive(Debug, Clone)]
struct Input {
    goal: u64,
    btns: Vec<u64>,
    joltage: Vec<u64>
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Key {
    goal: u64,
    btns: Vec<u64>,
}

fn calc_min_presses<'a>(goal: Vec<u64>, btns: &Vec<u64>, cnt: u64, cache: &'a mut HashMap<Key, u64>) -> (u64, bool, Vec<Vec<u64>>, HashMap<Key, u64>) {
    // println!("in Func");
    if cnt == 0 {
        // println!("\n fresh Start\n");
    }
    let this_goal = goal.iter().last().unwrap();
    if *this_goal == 0 {
        // we started with all the lights off
        println!("started with the goal, cnt: 0");
        let c = cache.clone();
        return (0, true, vec![goal], c);
    }
    // println!("goal: {:b}, cnt: {}", this_goal, cnt);
    let count = cnt + 1;

    let this_key = Key{goal: *goal.last().unwrap(), btns: btns.to_vec()};
    if cache.contains_key(&this_key) && (*cache.get(&this_key).unwrap() <= count) {
        // println!("using cache: count: {}", cache.get(&this_key).unwrap());
        let c = cache.clone();
        return (*cache.get(&this_key).unwrap(), true, vec![goal], c)
    }
    let mut updated_goal: Vec<Vec<u64>> = Vec::with_capacity(btns.len());
    // loop through the filtered buttons
    for b in btns {
        // println!("for button: {:b}", b);
        let new_goal = this_goal ^ b;
        let mut gg = goal.clone();
        gg.push(new_goal);
        updated_goal.push(gg);
    }
    if updated_goal.iter().any(|g|*g.last().unwrap() == 0) {
        println!("goal reached with cnt: {}", count);
        // updated the cache with this information
        let these_keys: Vec<Key> = goal.into_iter().map(|g|Key{goal: g, btns: btns.to_vec()}).collect();
        for k in these_keys {
            let prev_count = cache.get(&k).unwrap_or(&u64::MAX);
            cache.insert(k, count.min(*prev_count));
        }
        let c = cache.clone();
        return (count, true, updated_goal, c)
    }
    if updated_goal.iter().any(|g|goal.contains(g.last().unwrap())) {
        // we went in a loop, so exit here with "infinite" count
        // println!("removing loops");
        updated_goal = updated_goal.into_iter().filter(|g|!goal.contains(g.last().unwrap())).collect();
    }

    let c = cache.clone();
    return (count, false, updated_goal, c);
    // for g in updated_goal {
    //     let tmp_cnt = calc_min_presses(g, &btns, count, cache);
    //     // println!("returned with a cnt: {}", tmp_cnt);
    //     if tmp_cnt < min_count {
    //         // println!("updating min_count");
    //         min_count = tmp_cnt;
    //     }
    // }
    // min_count
}


pub fn main(scenario: &String, input_root: String) {
    let input_file = input_root + "/day_10_" + scenario + ".txt";
    let input = fs::read_to_string(input_file).expect("Failed to read input file");

    // split the input into the goal, buttons, and joltage requirements
    // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    let mut input_vals: Vec<Input> = Vec::new();
    for line_data in input.lines().map(|l|l.split(" ").collect()).collect::<Vec<Vec<&str>>>() {
        let mut this_line = Input { goal: 0, btns: vec![], joltage: vec![] };
        for group in line_data {
            match group.chars().nth(0) {
                Some(c) =>{
                    // println!("for group: {}, c: {}", group, c);
                    match c {
                        '[' => {
                            // println!("processing Goal...");
                            this_line.goal = group[1..(group.len()-1)]
                                            .chars().map(|c|c == '#').collect::<Vec<bool>>().iter().enumerate()
                                            .fold(0, |acc, (idx, b)|acc + 2u64.pow(idx as u32) * (*b as u64));
                            // println!("Goal: {}", this_line.goal);
                            },
                        '(' => {
                            // println!("processing Buttons...");
                            this_line.btns.push(group[1..(group.len()-1)]
                                            .split(",")
                                            .fold(0u64,|acc, n|acc + 2u64.pow(n.parse::<u32>().unwrap())));
                            // println!("Btns: {:?}", this_line.btns);
                            },
                        '{' => {
                            // println!("processing Joltages...");
                            this_line.joltage = group[1..(group.len()-1)]
                                            .split(",")
                                            .map(|n|n.parse::<u64>().unwrap()).collect();
                            // println!("Joltages: {:?}", this_line.joltage);
                            },
                        _ => {
                                println!("Uh Oh: {}", c);
                            }
                    }
                }
                None => {}
            }
        }
        // sort the buttons
        this_line.btns.sort();
        input_vals.push(this_line);
    }
    // println!("{:?}", input_vals);
    let cache: &mut HashMap<Key, u64> = &mut HashMap::new();
    let mut statuses: Vec<bool> = vec![false];
    let mut results: Vec<(u64, bool, Vec<Vec<u64>>, HashMap<Key, u64>)> = Vec::new();
    let mut counts: Vec<u64> = vec![0; input_vals.len()];
    let mut min_counts: Vec<u64> = Vec::new();
    // println!("num inputs: {}", input_vals.len());
    while statuses.into_iter().any(|s|!s) {
        println!("looping");
        // println!("{} inputs vals: {:?}", input_vals.len(), input_vals);
        // println!("{} counts: {:?}", counts.len(), counts);
        for (idx, inp) in input_vals.iter().enumerate() {
            let local_counts: u64 = *counts.iter().nth(idx).unwrap();
            // println!("local_counts: {:?}", local_counts);
            let r = calc_min_presses(vec![inp.goal], &inp.btns, local_counts, cache);
            results.push(r);
        } 
        results = zip(input_vals.clone(), &counts).map(|(m, cnt)|calc_min_presses(vec![m.goal], &m.btns, *cnt, cache))
                                                       .map(|(c, status, g, cache)|(c, status, g, cache)).collect();
        println!("num results: {}", results.len());
        let mut ids: Vec<usize> = Vec::new();
        for id in 0..results.len() {
            for _ in 0..(results.iter().nth(id).unwrap().2.len()) {
                ids.push(id);
            }
        }
        // println!("ids: {:?}", ids);

        statuses = results.clone().into_iter().map(|(_, s, _, _)|s).collect();
        println!("{}/{} statuses true", statuses.iter().fold(0u64, |acc, s|acc + (*s as u64)), statuses.len());
        // println!("any success: {:?}", statuses);
        println!("{} results before", results.len());
        for (s, r) in zip(statuses.clone().iter().enumerate(), results.clone().iter().enumerate()) {
            if *s.1 {
                // println!("status is true; min_counts {}", r.1.0);
                min_counts.push(r.1.0);
                // println!("removing results: {:?}", results);
                let this_id = ids.iter().nth(s.0).unwrap();
                println!("this_id: {}", this_id);
                for (idx, _) in statuses.iter().enumerate().filter(|(ind, _)|ind == this_id).rev() {
                    results.remove(idx);
                    println!("removing results: {}", idx);
                }
            }
        }
        println!("{} results after", results.len());
        let input_vals_2d: Vec<Vec<Input>> = zip(results.clone(), input_vals).map(|(res, inp)|res.2.iter().map(|goals|Input{goal: *goals.last().unwrap(), btns: inp.btns.clone(), joltage: inp.joltage.clone()}).collect::<Vec<Input>>()).collect();
        let input_lengths: Vec<usize> = input_vals_2d.iter().map(|i|i.len()).collect();
        counts = zip(results.clone(), input_lengths).map(|(r, l)|vec![r.0; l]).flatten().collect();
        input_vals = input_vals_2d.into_iter().flatten().collect();
        // println!("input_vals: {:?}", input_vals);
        // println!("statuses: {:?}", statuses);
        results.clear();
    }
    // println!("input_vals: {:?}", input_vals);
    println!("min counts: {:?}", counts);
    println!("total: {}", counts.iter().sum::<u64>());
    // println!("results: {:?}", results);

    // println!("fewest button presses: {}", input_vals.iter().fold(0, |acc, m|acc + calc_min_presses(vec![m.goal], &m.btns, 0, cache)));
    // // calculate the areas once between each pair of unique coordinates
    // let mut max_area:u64 = 0;
    // for (t1_idx, t1) in coords[..coords.len()-1].iter().enumerate() {
    //     for t2_idx in (t1_idx+1)..coords.len() {
    //         let this_area = rect_area(t1, &coords[t2_idx]);
    //         if this_area > max_area {
    //             max_area = this_area;
    //         }
    //     }
    // }
    // println!("The max area is {}", max_area);
}
