use std::fs;

pub fn main(scenario: &String, input_root: String) {
    let input_file = input_root + "/day_4_" + scenario + ".txt";
    let input = fs::read_to_string(input_file).expect("Failed to read input file");

    // replace the characters in the string with 0s and 1s to represent empty spaces and rolls
    let numerical_input_str = input.replace("@", "1")
                                .replace(".", "0");
    // parse the input into a matrix of 0s and 1s
    let mut matrix: Vec<Vec<u8>>= vec![];
    for line in numerical_input_str.lines() {
        matrix.push(line.chars().filter_map(|c|c.to_string().parse::<u8>().ok()).collect());
    }
    // predefine the neighbor locations as relative offsets
    let neighbors_ind = [[-1i8, -1], [-1, 0], [-1, 1],  [0, -1], [0, 1],  [1, -1], [1, 0], [1, 1]];
    let mut removed_rolls = 0;
    let mut next_state = matrix.clone();
    let mut current_state = matrix.clone();
    loop {
        let mut accessible_rolls = 0;
        // go through each element in the matrix and sum up all its neighbors
        for (r_idx, row) in current_state.iter().enumerate() {
            for (c_idx, element) in row.iter().enumerate() {
            // if the current element isn't a roll, skip it
                if *element == 0 {
                    continue;
                }
                let mut neighbor_sum = 0;
                for neighbor_i in neighbors_ind {
                    let nr_idx = r_idx as i32 + neighbor_i[0] as i32;
                    let nc_idx = c_idx as i32 + neighbor_i[1] as i32;
                    // using Vec.get() here which will return None for out of bound elements
                    let neighbor_val= match current_state.get(nr_idx as usize) {
                        Some(col) => {
                            match col.get(nc_idx as usize) {
                                Some(val) => *val,
                            None => 0,
                            }
                        },
                    None => 0,
                    };
                    neighbor_sum += neighbor_val;
                }
                // if there are few enough neighboring rolls, increment the counter
                // and remove the roll from the next state
                if neighbor_sum < 4 {
                    next_state[r_idx][c_idx] = 0;
                    accessible_rolls += 1;
                }
            }
        }
        // if there weren't any accessible rolls, break
        if accessible_rolls == 0 {
            break;
        }
        // accumulate all the accessible/removed rolls
        removed_rolls += accessible_rolls;
        // copy the next state into the current state and restart the loop
        current_state = next_state.clone();
    }
    println!("The number of removed rolls is {}", removed_rolls);
}
