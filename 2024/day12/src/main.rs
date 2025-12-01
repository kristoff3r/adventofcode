use std::collections::{HashMap, HashSet};

use util::{glam::ivec2, Grid};

fn main() {
    // let grid = Grid::<10, 10>::from_file("example.txt").unwrap();
    let grid = Grid::<140, 140>::from_file("in.txt").unwrap();

    // grid.print();

    let mut positions = HashMap::new();
    let mut regions = Vec::new();

    let mut cur_region_no = 0;
    let mut cur_region = Vec::new();
    let mut cur_char = grid[ivec2(0, 0)];
    let mut queue = vec![ivec2(0, 0)];
    'outer: loop {
        while let Some(pos) = queue.pop() {
            for (pos, v) in grid.neighbors(pos) {
                if positions.contains_key(&pos) {
                    continue;
                }
                if v == cur_char {
                    positions.insert(pos, cur_region_no);
                    cur_region.push(pos);
                    queue.push(pos);
                }
            }
        }

        regions.push(cur_region);

        // println!("finished region {cur_region}");
        for (pos, cell) in grid.iter_all() {
            if !positions.contains_key(&pos) {
                cur_region_no += 1;
                cur_char = cell;
                queue = vec![pos];
                positions.insert(pos, cur_region_no);
                cur_region = vec![pos];
                // println!("starting region {cur_region} at {pos}");
                continue 'outer;
            }
        }

        break;
    }

    println!(
        "regions: {}",
        positions
            .values()
            .copied()
            .collect::<HashSet<usize>>()
            .len()
    );

    let mut res1 = 0;
    for (region_no, region) in regions.iter().enumerate() {
        let mut area = 0;
        let mut perimeter = 0;
        // println!("{region:?} {}", region.len());
        for pos in region {
            let neighbors = grid
                .neighbors(*pos)
                .filter(|(pos, _v)| positions.get(pos) == Some(&region_no))
                .count();
            area += 1;
            perimeter += 4 - neighbors;
        }
        println!("{area} * {perimeter} == {}", area * perimeter);
        res1 += area * perimeter;
    }
    println!("{res1}");

    for (region_no, region) in regions.iter_mut().enumerate() {
        let mut sides = 0;
        while !region.is_empty() {
            let pos = region.pop().unwrap();

            while let Some(pos) = grid.get(pos + ivec2(1, 0)) {}
        }
    }
}
