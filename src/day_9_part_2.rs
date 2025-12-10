use std::fs;

#[derive(Debug)]
struct Area {
    area: u64,
    c1: Vec<f64>,
    c2: Vec<f64>,
}

#[derive(Debug)]
struct Edge(Vec<f64>, Vec<f64>);

fn rect_area(c1: &Vec<f64>, c2:&Vec<f64>) -> Area {
    let area = (((c1[0] - c2[0]).abs() + 1.0) * ((c1[1] - c2[1]).abs() + 1.0)) as u64;
    // println!("{:?}", Area{area, c1, c2});
    Area{area, c1:c1.to_vec(), c2:c2.to_vec()}
}

fn area_in_perimeter(a: &Area, vert_lines: &Vec<Edge>, horiz_lines: &Vec<Edge>) -> bool {
    // check four corners and then check for edges that cross the four edges

    let x_min = a.c1[0].min(a.c2[0]);
    let x_max = a.c1[0].max(a.c2[0]);
    let y_min = a.c1[1].min(a.c2[1]);
    let y_max = a.c1[1].max(a.c2[1]);

    let corners: [Vec<f64>; 4] = [
        vec![x_min, y_min],
        vec![x_min, y_max],
        vec![x_max, y_max],
        vec![x_max, y_min]];

    // check if corners have some vertical line to their left and right
    if !(corners.iter().fold(true, |acc, c|acc && vert_lines.iter().any(|e|
            e.0[0] <= c[0] &&
            e.0[1].min(e.1[1]) <= c[1] &&
            e.0[1].max(e.1[1]) >= c[1])) &&
         corners.iter().fold(true, |acc, c|acc && vert_lines.iter().any(|e|
            e.0[0] > c[0] &&
            e.0[1].min(e.1[1]) <= c[1] &&
            e.0[1].max(e.1[1]) >= c[1]))) {
        return false;
    }

    let vert_edges: [Edge; 2] = [
        Edge(vec![x_min, y_min],
             vec![x_min, y_max]),
        Edge(vec![x_max, y_max],
             vec![x_max, y_min])];

    let horiz_edges: [Edge; 2] = [
        Edge(vec![x_min, y_max],
             vec![x_max, y_max]),
        Edge(vec![x_max, y_min],
             vec![x_min, y_min])];

    // check vertical edges for horizontal perimeter crossings
    // println!("Horizontal edges");
    for h in horiz_edges {
        for v in vert_lines {
            // println!("testing {:?} and {:?}", h, v);
            if v.0[0] > h.0[0].min(h.1[0]) &&
               v.0[0] < h.0[0].max(h.1[0]) &&
               v.0[1].min(v.1[1]) < h.0[1] &&
               v.0[1].max(v.1[1]) > h.0[1] {
                // if the vertical line crosses the horizontal edge
                // this area is not entirely within the perimeter
                return false
            }
        }
    }

    // check horizontal edges for vertical perimeter crossings
    // println!("Vertical edges");
    for v in vert_edges {
        for h in horiz_lines {
            // println!("testing {:?} and {:?}", h, v);
            if v.0[0] > h.0[0].min(h.1[0]) &&
               v.0[0] < h.0[0].max(h.1[0]) &&
               v.0[1].min(v.1[1]) < h.0[1] &&
               v.0[1].max(v.1[1]) > h.0[1] {
                // if the horizontal line crosses the vertical edge
                // this area is not entirely within the perimeter
                return false
            }
        }
    }

    true
}

pub fn main(scenario: &String, input_root: String) {
    let input_file = input_root + "/day_9_" + scenario + ".txt";
    let input = fs::read_to_string(input_file).expect("Failed to read input file");

    // split the input into a vector of coordinates (Vec[x, y])
    let coords: Vec<Vec<f64>> = input.lines().map(|l|l.split(",").map(|c|c.parse::<f64>().unwrap()).collect()).collect();
    // calculate the areas once between each pair of unique coordinates
    let mut areas:Vec<Area> = Vec::new();
    for (t1_idx, t1) in coords[..coords.len()-1].iter().enumerate() {
        for t2_idx in (t1_idx+1)..coords.len() {
            areas.push(rect_area(t1, &coords[t2_idx]));
        }
    }
    areas.sort_by(|a, b| b.area.cmp(&a.area));

    // add one last coord to complete the loop
    let mut coord_closed = coords.clone();
    coord_closed.push(coords[0].clone());

    // process perimeter segments into horizontal and vertical lines
    let vert_perimeter_lines: Vec<Edge> = coord_closed[..(coord_closed.len() - 1)].iter().enumerate()
        .filter(|(idx, pt1)|pt1[0] == coord_closed[idx+1][0])
        .map(|(idx, pt1)|Edge(pt1.to_vec(), coord_closed[idx+1].clone())).collect();
    let horiz_perimeter_lines: Vec<Edge> = coord_closed[..(coord_closed.len() - 1)].iter().enumerate()
        .filter(|(idx, pt1)|pt1[1] == coord_closed[idx+1][1])
        .map(|(idx, pt1)|Edge(pt1.to_vec(), coord_closed[idx+1].clone())).collect();

    // loop through the areas and stop at the largest one whose points are contained
    for a in areas {
        // check four corners and then check for edges that cross the four edgespri
        if area_in_perimeter(&a, &vert_perimeter_lines, &horiz_perimeter_lines) {
            println!("The max area is {}", a.area);
            return
        }
        else {
        }
    }
}
