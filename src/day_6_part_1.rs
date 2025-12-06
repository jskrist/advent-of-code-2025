use std::fs;

fn transpose<T: Copy>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    (0..len).map(|i| v.iter().map(|row| row[i]).collect()).collect()
}

pub fn main(scenario: &String, input_root: String) {
    let input_file = input_root + "/day_6_" + scenario + ".txt";
    let input = fs::read_to_string(input_file).expect("Failed to read input file");

    let mut vals: Vec<Vec<i64>> = Vec::new();
    let mut operators: Vec<&str> = Vec::new();
    for line in input.lines() {
        let line_chars = line.split_whitespace();
        let line_chars_copy = line_chars.clone();

        let line_vals = line_chars.map(|c| c.parse::<i64>().unwrap_or(0i64)).collect();
        vals.push(line_vals);
        operators = line_chars_copy.collect();
    }
    vals.pop();
    let vals_t = transpose(&vals);
    let mut results: Vec<i64> = Vec::with_capacity(vals.len());
    for (idx, op) in operators.iter().enumerate() {
        match *op {
            "+" => {
                results.push(vals_t[idx].iter().sum::<i64>());
            }
            "*" => {
                results.push(vals_t[idx].iter().product::<i64>());
            }
            _ => {}
        }
    }

    let grand_total:i64 = results.iter().sum();

    println!("The grand total is {}", grand_total);
}
