use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    goal: u64,
    btns: Vec<Vec<u64>>,
    joltage: Vec<u64>,
    id: u64
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Key {
    id: u64,
    joltage: Vec<u64>,
}

// impl PartialEq for Input {
//     fn eq(&self, other: &Input) -> bool {
//         (self.id == other.id) &&
//         (self.joltage == other.joltage)
//     }
// }

fn apply_button(input: Input, cache: &mut HashMap<Key, u64>) -> u64 {
    let btns: Vec<Vec<u64>> = input.btns.clone();
    let joltage: Vec<u64> = input.joltage.clone();
    // println!("joltage: {:?}", joltage);
    // find out where the joltage counters are already zero
    let zero_ind: Vec<usize> = joltage.iter()
       .enumerate()
       .filter(|&(_, item)| *item == 0)
       .map(|(index, _)| index)
       .collect();
    // filter out ineffective buttons (those whose joltage is already correct)
    let mut filtered_buttons: Vec<Vec<u64>> = btns.clone().into_iter().filter(|b|!zero_ind.iter().any(|z_idx|b.contains(&(*z_idx as u64)))).collect();
    // if any of the remaining joltage counters don't exists in the remaining buttons, then we have hit an impasse
    let still_possible = joltage.iter().enumerate().all(|jc|filtered_buttons.iter().any(|b|b.contains(&(jc.0 as u64))));
    if !still_possible {
        println!("remaining buttons can't make joltage!");
        return 0;
    }
    let max_joltage = joltage.iter().filter(|j|**j != 0).max().unwrap();
    // println!("max joltage: {}", max_joltage);
    let max_ind = joltage.iter().position(|j|j == max_joltage).unwrap() as u64;
    // put the largest buttons that contain the smallest joltage counter first
    filtered_buttons.sort_by(|a, b|b.len().cmp(&a.len()));
    filtered_buttons.sort_by(|a, b|b.contains(&max_ind).cmp(&a.contains(&max_ind)));

    // println!("btns: {:?}", filtered_buttons);

    // for b in filtered_buttons.iter() {
    for (_, b) in filtered_buttons.iter().enumerate() {
        // println!("pressing button {}/{}", idx, filtered_buttons.len()-1);
        let mut local_joltage: Vec<u64> = input.joltage.clone();
        // pressing the button reduces associated counters
        for j_idx in b {
            local_joltage[*j_idx as usize] -= 1;
        }
        println!("pressed {:?} => local_joltage: {:?}", b, local_joltage);
        let this_key = Key{id: input.id, joltage: local_joltage.clone()};
        if cache.contains_key(&this_key) {
            println!("cache hit, returning {}", *cache.get(&this_key).unwrap());
            return *cache.get(&this_key).unwrap();
        }
        cache.insert(this_key.clone(), 0);
        if local_joltage.iter().sum::<u64>() != 0 {
            // add to the next set of inputs
            let count = apply_button(Input{
                                    goal: input.goal.clone(),
                                    btns: btns.clone(),
                                    joltage: local_joltage.clone(),
                                    id: input.id}, cache);
            if count > 0 {
                cache.insert(this_key, count + 1);
                return count + 1;
            }
        }
        else {
            println!("found a solution for {}", input.id);
            cache.insert(this_key, 1);
            return 1;
        }
    }
    0
}

pub fn main(scenario: &String, input_root: String) {
    let input_file = input_root + "/day_10_" + scenario + ".txt";
    let input = fs::read_to_string(input_file).expect("Failed to read input file");
    let mut cache: HashMap<Key, u64> = HashMap::new();

    // split the input into the goal, buttons, and joltage requirements
    let mut input_vals: Vec<Input> = Vec::new();
    for (line_idx, line_data) in input.lines().enumerate().map(|(idx, l)|(idx, l.split(" ").collect())).collect::<Vec<(usize, Vec<&str>)>>() {
        let mut this_line = Input { goal: 0, btns: vec![], joltage: vec![], id: line_idx as u64 };
        for group in line_data {
            match group.chars().nth(0) {
                Some(c) =>{
                    match c {
                        '[' => {
                            this_line.goal = group[1..(group.len()-1)]
                                            .chars().map(|c|c == '#').collect::<Vec<bool>>().iter().enumerate()
                                            .fold(0, |acc, (idx, b)|acc + 2u64.pow(idx as u32) * (*b as u64));
                            },
                        '(' => {
                            this_line.btns.push(group[1..(group.len()-1)]
                                            .split(",")
                                            .map(|n|n.parse::<u64>().unwrap()).collect());
                            },
                        '{' => {
                            this_line.joltage = group[1..(group.len()-1)]
                                            .split(",")
                                            .map(|n|n.parse::<u64>().unwrap()).collect();
                            },
                        _ => {
                                println!("Uh Oh: {}", c);
                            }
                    }
                }
                None => {}
            }
        }
        input_vals.push(this_line);
    }

    // start main algorithm (breadth first search)
    let counts: Vec<u64> = input_vals.into_iter().map(|input|apply_button(input.clone(), &mut cache)).collect();
    println!("counts: {:?}", counts);
    println!("The minimum button presses is {}", counts.iter().sum::<u64>());
}
