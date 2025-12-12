use std::fs;

#[derive(Debug, Clone)]
struct Input {
    goal: u64,
    btns: Vec<u64>,
    joltage: Vec<u64>,
    id: u64
}

fn apply_buttons(inputs: Input) -> Vec<Input> {
    let goal: u64 = inputs.goal;
    let btns: Vec<u64> = inputs.btns.clone();
    let mut used_btns: Vec<u64> = Vec::with_capacity(btns.len());
    let mut next_inputs: Vec<Input> = Vec::with_capacity(btns.len());
    for b in btns {
        used_btns.push(b);
        // pressing the button is an xor
        next_inputs.push(Input{
            goal: goal ^ b,
            btns: inputs.btns.clone().into_iter().filter(|btn|!used_btns.contains(btn)).collect(),
            joltage: inputs.joltage.clone(),
            id: inputs.id});
    }
    next_inputs
}

fn check_results_and_update_outputs(results: &mut Vec<Input>, min_pushes: &mut Vec<u64>, counts: u64) {
    // update output
    let completed_ids: Vec<u64> = results.iter().filter(|inputs|inputs.goal == 0)
                                                .map(|inputs|inputs.id).collect();
    for rg_inputs in results.iter().filter(|inputs|inputs.goal == 0) {
        min_pushes[rg_inputs.id as usize] = counts;
    }
    let mut to_remove: Vec<usize> = Vec::with_capacity(results.len());
    for id in completed_ids {
        // remove all Inputs for completed ids
        for (idx, input) in results.iter().enumerate() {
            if input.id == id && !to_remove.contains(&idx){
                to_remove.push(idx);
            }
        }
    }

    for idx in to_remove.into_iter().rev() {
        results.remove(idx);
    }
}

pub fn main(scenario: &String, input_root: String) {
    let input_file = input_root + "/day_10_" + scenario + ".txt";
    let input = fs::read_to_string(input_file).expect("Failed to read input file");

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
                                            .fold(0u64,|acc, n|acc + 2u64.pow(n.parse::<u32>().unwrap())));
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
        // sort the buttons
        this_line.btns.sort();
        input_vals.push(this_line);
    }

    let mut min_pushes: Vec<u64> = vec![u64::MAX; input_vals.len()];
    let mut counts = 0;
    // check for trivial case where we start with the solution first
    check_results_and_update_outputs(&mut input_vals, &mut min_pushes, counts);
    // start main algorithm (breadth first search)
    let mut updated_inputs: Vec<Input> = input_vals;

    while updated_inputs.len() > 0 {
        // apply a single button press for each of the buttons available
        updated_inputs = updated_inputs.into_iter().map(|inputs|apply_buttons(inputs)).flatten().collect();
        counts += 1;
        println!("counts: {}", counts);

        // check if any of the button presses caused us to reach our goal
        // remove those inputs and add the number of presses required to reach the goal
        check_results_and_update_outputs(&mut updated_inputs, &mut min_pushes, counts);
        println!("{} inputs left", updated_inputs.len());
    }

    // report out results 
    println!("The minimum button presses is {}", min_pushes.iter().sum::<u64>());
}
