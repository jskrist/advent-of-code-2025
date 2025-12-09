use std::fs;
// use std::cmp::min;
use std::iter::zip;
use std::collections::HashSet;
use std::cmp::Ordering;

#[derive(Debug, PartialOrd, PartialEq)]
struct Dist {
    dist: f64,
    p1: usize,
    p2: usize,
}

impl Eq for Dist {}

impl Ord for Dist {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        println!("Ordering dist {:?} vs {:?}", self, other);
        if self.dist < other.dist {
            Ordering::Less
        } else if self.dist > other.dist {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

fn dist(jcn1: &Vec<f64>, jcn2: &Vec<f64>) -> f64 {
    let d: f64 = zip(jcn1, jcn2).map(|(p1, p2)| {
        let tmp = p2 - p1;
        (tmp * tmp) as f64
        }).sum();
    d.sqrt()
}

pub fn main(scenario: &String, input_root: String) {
    let input_file = input_root + "/day_8_" + scenario + ".txt";
    let input = fs::read_to_string(input_file).expect("Failed to read input file");

    let connections_to_make = 10;
    // split the input into a vector of coordinates (Vec[x, y, z])
    let coords: Vec<Vec<f64>> = input.lines().map(|l|l.split(",").map(|c|c.parse::<f64>().unwrap()).collect()).collect();
    // calculate the distances once between each pair of unique coordinates
    // let mut dists: Vec<Dist> = Vec::new();
    let mut dists: Vec<Dist> = coords[..coords.len()-1].iter().enumerate()
                                                       .map(|(p1_idx, p1)|((p1_idx+1)..coords.len())
                                                       .map(|p2_idx|Dist{p1:p1_idx, p2:p2_idx, dist: dist(p1, &coords[p2_idx])})
                                                       .collect::<Vec<Dist>>()).flatten().collect();
    // for (p1_idx, p1) in coords[..coords.len()-1].iter().enumerate() {
    //     for p2_idx in (p1_idx+1)..coords.len() {
    //         dists.push(Dist{p1:p1_idx, p2:p2_idx, dist: dist(p1, &coords[p2_idx])});
    //         // println!("distance from {} to {} is {}", p1_idx, p2_idx, dists.last().unwrap().dist);
    //     }
    // }
    // println!("min dist: {}", min_dist);
    // sort distances to get the closest pairs
    dists.sort();
    // println!("Dists:");
    // for d in &dists {
    //     println!("{}", d.dist);
    // }
    // start making circuits
    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    let mut num_connections = 0;
    for d in &dists {
        // println!("checking {:?}", d);
        // println!("Coords {:?}, {:?}", coords[d.p1], coords[d.p2]);
        if circuits.iter().any(|v|v.contains(&d.p1) & v.contains(&d.p2)) {
            // skip this pair, as it is already in a circuit
            // println!("skipping {:?}, because {} and {} are already in a circuit together.", d, d.p1, d.p2);
            // println!("Circuits: {:?}", circuits);
            num_connections += 1;
            if num_connections == connections_to_make {
                // println!("Breaking out");
                break;
            }
            continue;
        }
        let mut pushed = false;
        let mut first_circuit: &mut HashSet<usize> = &mut HashSet::new();
        for c in &mut circuits {
            // println!("in circuit {:?}", c);
            if c.contains(&d.p1) {
                // println!("found {} in circuit", d.p1);
                if !pushed {
                    // println!("inserting {} into circuit", d.p2);
                    c.insert(d.p2);
                    pushed = true;
                    first_circuit = c;
                }
                else {
                    // need to join the two circuits
                    // println!("joining {:?} and {:?}", c, first_circuit);
                    c.extend(first_circuit.iter());
                    first_circuit.clear();
                    break;
                }
            }
            else if c.contains(&d.p2) {
                // println!("found {} in circuit", d.p2);
                if !pushed {
                    // println!("inserting {} into circuit", d.p1);
                    c.insert(d.p1);
                    pushed = true;
                    first_circuit = c;
                }
                else {
                    // need to join the two circuits
                    // println!("joining {:?} and {:?}", c, first_circuit);
                    c.extend(first_circuit.iter());
                    first_circuit.clear();
                    break;
                }
            }
        }
        if !pushed {
            // println!("pushing {:?} to new circuit", d);
            // println!("coords {:?} and {:?}", coords[d.p1], coords[d.p2]);
            circuits.push(HashSet::from([d.p1, d.p2]));
        }
        else {
            // println!("joined coords {:?} and {:?}", coords[d.p1], coords[d.p2]);
        }
        num_connections += 1;
        if num_connections == connections_to_make {
            // println!("Breaking out");
            break;
        }
    }

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    // println!("{:?}", circuits);
    // println!("Circuit lengths:");
    // for c in &circuits {
    //     println!("{}", c.len());
    // }
    println!("The answer is {}", circuits[..3].iter().fold(1, |acc, c|acc * c.len()));
}
