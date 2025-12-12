use std::fs;

#[derive(Debug, Clone, Eq)]
struct Input {
    goal: u64,
    btns: Vec<Vec<u64>>,
    joltage: Vec<u64>,
    id: u64
}

impl PartialEq for Input {
    fn eq(&self, other: &Input) -> bool {
        (self.id == other.id) &&
        (self.joltage == other.joltage)
    }
}

fn apply_buttons(inputs: Input) -> Vec<Input> {
    let btns: Vec<Vec<u64>> = inputs.btns.clone();
    let mut next_inputs: Vec<Input> = Vec::with_capacity(btns.len());
    let joltage: Vec<u64> = inputs.joltage.clone();
    
    // find out where the joltage counters are already zero
    let zero_ind: Vec<usize> = joltage.iter()
       .enumerate()
       .filter(|&(_, item)| *item == 0)
       .map(|(index, _)| index)
       .collect();
    if zero_ind.len() > 0 {
        // println!("joltage: {:?}\nzero_ind: {:?}", joltage, zero_ind);
    }
    // filter out ineffective buttons (those whose joltage is already correct)
    let filtered_buttons: Vec<Vec<u64>> = btns.clone().into_iter().filter(|b|!zero_ind.iter().any(|z_idx|b.contains(&(*z_idx as u64)))).collect();

    for b in &filtered_buttons {
        let mut local_joltage: Vec<u64> = inputs.joltage.clone();
        // println!("starting joltage: {:?}", local_joltage);
        // println!("pressing button: {:?}", b);
        // pressing the button reduces associated counters
        for j_idx in b {
            local_joltage[*j_idx as usize] -= 1;
        }
        // println!("ending joltage: {:?}", local_joltage);

        // add to the next set of inputs
        next_inputs.push(Input{
            goal: inputs.goal.clone(),
            btns: filtered_buttons.clone(),
            joltage: local_joltage,
            id: inputs.id});
    }
    next_inputs
}

fn check_results_and_update_outputs(results: &mut Vec<Input>, min_pushes: &mut Vec<u64>, counts: u64) {
    // remove all duplicate Input states
    let mut to_remove: Vec<usize> = Vec::with_capacity(results.len());
    for (idx1, input1) in results[..(results.len()-1)].iter().enumerate() {
        for (idx2, input2) in results[(idx1+1)..].iter().enumerate() {
            if input1 == input2  && !to_remove.contains(&(idx1 + 1 + idx2)){
                // println!("{:?} ==\n{:?} =>\n{:?}", input1, input2, results[(idx1 + 1 + idx2) as usize]);
                to_remove.push(idx1 + 1 + idx2);
            }
        }
    }
    to_remove.sort();
    // // println!("sorted to_remove: {:?}", to_remove);
    // to_remove.dedup();
    // // println!("deduped to_remove: {:?}", to_remove);
    // let mut removal_ids: Vec<_> = to_remove.iter().map(|idx|results[*idx as usize].id).collect();
    // removal_ids.sort();
    // removal_ids.dedup();
    // println!("removal_ids1: {:?}", removal_ids);
    for idx in to_remove.iter().rev() {
        results.remove(*idx);
    }
    // println!("found {} dups, {} results remaining", to_remove.len(), results.len());
    to_remove.clear();

    // // check for results that contain completed tasks
    // let mut joltage_sum: Vec<_> = results.iter().filter(|input|input.id == 0).map(|inputs|inputs.joltage.iter().sum::<u64>()).collect();
    // joltage_sum.sort();
    

    // if joltage_sum.len() > 0 {
    //     // println!("joltage_sum: {:?}", joltage_sum[0]);
    // }
    // else {
    //     let mut ids = results.iter().map(|input|input.id).collect::<Vec<u64>>();
    //     ids.sort();
    //     ids.dedup();
    //     // println!("no results left for id == 0?: {:?}", ids);
    // }
    let completed_results: Vec<&Input> = results.iter().filter(|inputs|inputs.joltage.iter().sum::<u64>() == 0).collect();
    for rg_inputs in &completed_results {
        // update the output for the completed task
        println!("{} was completed with {} pushes", rg_inputs.id, counts);
        min_pushes[rg_inputs.id as usize] = counts;
    }
    // remove the results associated with the id of any of the completed tasks
    let completed_ids: Vec<u64> = completed_results.iter().map(|inputs|inputs.id).collect();
    for id in completed_ids {
        // remove all Inputs for completed ids
        for (idx, input) in results.iter().enumerate() {
            if input.id == id && !to_remove.contains(&idx) {
                to_remove.push(idx);
            }
        }
    }
    to_remove.sort();
    // removal_ids = to_remove.iter().map(|idx|results[*idx as usize].id).collect();
    // removal_ids.sort();
    // removal_ids.dedup();
    // println!("removal_ids2: {:?}", removal_ids);
    for idx in to_remove.iter().rev() {
        results.remove(*idx);
    }
    to_remove.clear();
    // println!("{} results remaining", results.len());
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
    println!("min_pushes: {:?}", min_pushes);
    // report out results 
    println!("The minimum button presses is {}", min_pushes.iter().sum::<u64>());
}
