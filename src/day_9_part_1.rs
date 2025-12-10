use std::fs;


fn rect_area(t1: &Vec<f64>, t2:&Vec<f64>) -> u64 {
    let area = ((t1[0] - t2[0] + 1.0).abs() * (t1[1] - t2[1] + 1.0).abs()) as u64;
    area
}

pub fn main(scenario: &String, input_root: String) {
    let input_file = input_root + "/day_9_" + scenario + ".txt";
    let input = fs::read_to_string(input_file).expect("Failed to read input file");

    // split the input into a vector of coordinates (Vec[x, y])
    let coords: Vec<Vec<f64>> = input.lines().map(|l|l.split(",").map(|c|c.parse::<f64>().unwrap()).collect()).collect();
    // calculate the areas once between each pair of unique coordinates
    let mut max_area:u64 = 0;
    for (t1_idx, t1) in coords[..coords.len()-1].iter().enumerate() {
        for t2_idx in (t1_idx+1)..coords.len() {
            let this_area = rect_area(t1, &coords[t2_idx]);
            if this_area > max_area {
                max_area = this_area;
            }
        }
    }
    println!("The max area is {}", max_area);
}
