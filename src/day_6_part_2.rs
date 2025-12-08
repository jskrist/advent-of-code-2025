use std::fs;

fn transpose<T: Copy>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    // transpose a generic 2D vector of data
    let len = v[0].len();
    (0..len).map(|i| v.iter().map(|row| row[i]).collect()).collect()
}

pub fn main(scenario: &String, input_root: String) {
    let input_file = input_root + "/day_6_" + scenario + ".txt";
    let input = fs::read_to_string(input_file).expect("Failed to read input file");

    // split each line into its individual characters
    let mod_lines: Vec<Vec<char>> = input.lines().map(|l|l.chars().collect()).collect();
    // let mut mod_lines: Vec<Vec<char>> = Vec::new();
    // for line in input.lines() {
    //     mod_lines.push(line.chars().collect());
    // }
    // transpose the character matrix
    let new_nums = transpose(&mod_lines);
    // filter each line into its numeric characters and fold all the charachters on a given line into a single number
    // let vals:Vec<i64> = new_nums.iter()
    //                             .map(|n_vec| n_vec.iter()
    //                                                           .filter(|n|n.is_numeric())
    //                                                           .fold(0, |acc, v|acc*10 + v.to_string()
    //                                                                                                          .parse::<i64>()
    //                                                                                                          .unwrap_or(0)))
    //                             .collect();
    let vals: Vec<Vec<i64>> = new_nums.iter()
                                .map(|n_vec| n_vec.into_iter()
                                                              .filter_map(|n|n.to_string().parse::<i64>().ok())
                                                              .collect())
                                .collect();
    // split up the vector of numbers into each of the groups of numbers that need to be processed
    let chunked_vals: Vec<Vec<i64>> = vals.split(|v|v.len() == 0)
                                 .map(|s|s.into_iter()
                                                       .map(|v|v.iter().fold(0i64, |acc, d|acc * 10 + d))
                                                       .collect()).collect();
    // println!("chunked_vals: {:?}", chunked_vals);
    // using split() saves us from having to do the following:
    // let mut chunked_vals:Vec<Vec<i64>> = Vec::new();
    // chunked_vals.push(vec![]);
    // let mut ind: usize = 0;
    // for val in vals.clone() {
    //     if val == 0 {
    //         ind += 1;
    //         chunked_vals.push(vec![]);
    //         continue;
    //     }
    //     chunked_vals[ind].push(val);
    // }

    // gather the operators from the last line in the file
    let operators:Vec<&str> = input.lines().next_back().unwrap().split_whitespace().collect();
    // let mut operators: Vec<&str> = Vec::new();
    // for line in input.lines().rev().next().unwrap().split_whitespace().collect() {
    //     let line_chars = line.split_whitespace();
    //     operators = line_chars.collect();
    //     break;
    // }
    let results: Vec<i64> = operators.iter().enumerate().map(|(idx, op)| {
        match *op {
            "+" => {
                chunked_vals[idx].iter().sum::<i64>()
            }
            "*" => {
                chunked_vals[idx].iter().product::<i64>()
            }
            _ => {
                0i64
            }
        }
    }).collect();
    // let mut results: Vec<i64> = Vec::with_capacity(vals.len());
    // for (idx, op) in operators.iter().enumerate() {
    //     match *op {
    //         "+" => {
    //             results.push(chunked_vals[idx].iter().sum::<i64>());
    //         }
    //         "*" => {
    //             results.push(chunked_vals[idx].iter().product::<i64>());
    //         }
    //         _ => {}
    //     }
    // }

    let grand_total:i64 = results.iter().sum();

    println!("The grand total is {}", grand_total);
}
