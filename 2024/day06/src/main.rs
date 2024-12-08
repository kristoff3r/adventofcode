use std::collections::HashSet;

use util::{glam::IVec2, Direction, Grid};

fn main() {
    let mut grid = Grid::<130, 130>::from_file("in.txt").unwrap();

    // let mut grid = Grid::<10, 10>::from_file("example.txt").unwrap();
    // grid.print();

    let start_pos = grid.find(b'^').unwrap();

    let mut visited = solve(&grid, start_pos).unwrap();
    println!("{}", visited.len());
    visited.remove(&start_pos);

    let mut res2 = 0;
    for pos in visited {
        grid[pos] = b'O';

        if solve(&grid, start_pos).is_none() {
            res2 += 1;
        }

        grid[pos] = b'.';
    }

    println!("{res2}");
}

fn solve<const M: usize, const N: usize>(
    grid: &Grid<M, N>,
    start_pos: IVec2,
) -> Option<HashSet<IVec2>> {
    let mut direction = Direction::North;
    let mut cur_pos = start_pos;

    let mut visited = HashSet::new();
    visited.insert(cur_pos);

    let mut halted = false;

    for _ in 0..10000 {
        let step = direction.step();
        let new_pos = cur_pos + step;
        match grid.get(new_pos) {
            Some(b'.' | b'X' | b'^') => {
                cur_pos = new_pos;
                visited.insert(cur_pos);
            }
            Some(b'#' | b'O') => {
                direction = direction.clockwise();
            }
            None => {
                halted = true;
                break;
            }
            Some(tile) => panic!("invalid tile: {tile}"),
        }

        if cur_pos == start_pos && direction == Direction::North {
            return None;
        }
    }

    if halted {
        Some(visited)
    } else {
        None
    }
}
