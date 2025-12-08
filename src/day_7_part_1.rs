use std::fs;
use std::cmp::min;
use std::collections::HashSet;

pub fn main(scenario: &String, input_root: String) {
    let input_file = input_root + "/day_7_" + scenario + ".txt";
    let input = fs::read_to_string(input_file).expect("Failed to read input file");

    let lines: Vec<&str> = input.lines().collect();
    let mut tach_ind: HashSet<usize> = HashSet::new();
    let mut splitter_ind: Vec<usize> = Vec::new();
    let mut num_splits: u64 = 0;
    let line_len = lines[0].len();
    for line in lines {
        // S marks the origination point for a tachyon beam
        match line.find("S") {
            Some(ind) => {tach_ind.insert(ind);},
            None => (),
        };
        let matches = line.match_indices("^");
        for m in matches.clone() {
            let i = m.0;
            // if the ^ is not at a position being tracked, skip it
            if !tach_ind.contains(&i) {
                continue;
            }
            // remove index from tracked beams
            tach_ind.remove(&i);
            splitter_ind.push(i);
        };
        for i in splitter_ind.iter() {
            // add the adjacent indices to be tracked
            tach_ind.insert(i.saturating_sub(1));
            tach_ind.insert(min(i + 1, line_len));
            num_splits += 1;
        };
        splitter_ind.clear();
    }

    println!("The tachyon beam split {} times", num_splits);
}
