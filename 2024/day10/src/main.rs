use std::collections::{HashMap, HashSet};

use util::{glam::IVec2, Grid};

fn main() {
    let grid = Grid::<50, 50>::from_file("in.txt").unwrap();
    // let grid = Grid::<8, 8>::from_file("example.txt").unwrap();
    // let grid = Grid::<7, 7>::from_file("example1.txt").unwrap();

    let mut active = Vec::new();
    for (pos, v) in grid.iter_all() {
        if v == b'0' {
            active.push((pos, pos, v, HashSet::from([pos])));
        }
    }

    grid.print();

    let mut res1 = 0;
    let mut res2 = 0;
    let mut result: HashMap<IVec2, HashSet<IVec2>> = HashMap::new();
    while let Some((start_pos, cur_pos, v1, seen)) = active.pop() {
        for (new_pos, v2) in grid.neighbors(cur_pos) {
            let mut seen = seen.clone();
            if !seen.insert(new_pos) {
                continue;
            }

            if v1 + 1 == v2 {
                if v2 == b'9' {
                    res2 += 1;
                    result.entry(start_pos).or_default().insert(new_pos);
                } else {
                    active.push((start_pos, new_pos, v2, seen));
                }
            }
        }
    }

    for positions in result.values() {
        res1 += positions.len();
    }

    println!("{res1}");
    println!("{res2}");
}
