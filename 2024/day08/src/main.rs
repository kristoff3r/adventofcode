use std::collections::{HashMap, HashSet};

use util::{glam::IVec2, Grid};

// fn dist(pos1: (i32, i32), pos2: (i32, i32)) -> (i32, i32) {}

fn main() {
    let mut grid = Grid::<50, 50>::from_file("in.txt").unwrap();
    // let mut grid = Grid::<12, 12>::from_file("example.txt").unwrap();

    // grid.print();
    let mut values: HashMap<u8, Vec<IVec2>> = HashMap::new();

    for (pos, cell) in grid.iter_all() {
        if cell == b'#' || cell == b'.' {
            continue;
        }
        values.entry(cell).or_default().push(pos);
    }

    let mut locations = HashSet::new();
    let mut locations2 = HashSet::new();
    for (_v, locs) in values {
        for (n, pos1) in locs.iter().enumerate() {
            for pos2 in locs.iter().skip(n) {
                let dist = pos2 - pos1;
                let candidates = [pos1 + dist, pos1 - dist, pos2 + dist, pos1 - dist];
                for &c in candidates.iter().filter(|&c| c != pos1 && c != pos2) {
                    // println!(
                    //     "considering {c} because of {v} on {pos1:?} and {pos2:?}",
                    //     v = char::from_u32(v as u32).unwrap()
                    // );
                    if grid.in_bounds(c) {
                        // println!("{c} in bounds!");
                        locations.insert(c);
                    }
                }

                for dist in [dist, -dist] {
                    if dist == IVec2::ZERO {
                        break;
                    }
                    for mut p in [*pos1, *pos2] {
                        while grid.in_bounds(p) {
                            locations2.insert(p);
                            if grid[p] == b'.' {
                                grid[p] = b'#';
                            }
                            p += dist;
                        }
                    }
                }
            }
        }
    }

    // grid.print();

    println!("{}", locations.len());
    println!("{}", locations2.len());
}
