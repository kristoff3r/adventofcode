use std::collections::VecDeque;

use util::Grid;

fn main() {
    // let mut grid = Grid::<10, 10>::from_file("example.txt").unwrap();
    let mut grid = Grid::<135, 135>::from_file("in.txt").unwrap();

    let mut res1 = 0;
    let mut queue = VecDeque::new();
    for (pos, val) in grid.iter_all() {
        if val == b'@' {
            let count = grid.neighbors_8(pos).filter(|v| v.1 == b'@').count();
            if count < 4 {
                res1 += 1;
                queue.push_back(pos);
            }
        }
    }

    println!("{res1}");

    let mut res2 = 0;
    while let Some(pos) = queue.pop_front() {
        let val = grid[pos];
        if val == b'@' {
            let count = grid.neighbors_8(pos).filter(|v| v.1 == b'@').count();
            if count < 4 {
                grid[pos] = b'x';
                res2 += 1;

                queue.extend(
                    grid.neighbors_8(pos)
                        .filter_map(|(pos, val)| (val == b'@').then_some(pos)),
                );
            }
        }
    }

    println!("{res2}");
}
