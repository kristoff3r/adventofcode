use std::collections::{hash_map::Entry, HashMap};

type Pos = (i32, i32);

fn main() {
    let input = include_str!("../in.txt").as_bytes();
    // let input = include_str!("../example1.txt").as_bytes();

    let grid: Vec<_> = input
        .split(|&b| b == b'\n')
        .filter(|r| !r.is_empty())
        .collect();

    let mut start = (0, 0);
    for (y, row) in grid.iter().enumerate() {
        for (x, &b) in row.iter().enumerate() {
            if b == b'S' {
                start = (x as i32, y as i32);
            }
        }
    }

    let mut queue: Vec<(Pos, u8)> = vec![(start, b'S')];
    let mut distances: HashMap<Pos, i32> = HashMap::new();
    distances.insert(start, 0);

    while !queue.is_empty() {
        let (pos, from) = queue.pop().unwrap();
        let dist = distances[&pos];

        // println!("{pos:?} {}", from as char);
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let nx = pos.0 + dx;
            let ny = pos.1 + dy;
            if let Some(&to) = grid.get(ny as usize).and_then(|row| row.get(nx as usize)) {
                if legal_move(from, to, dx, dy) {
                    match distances.entry((nx, ny)) {
                        Entry::Occupied(mut e) => {
                            let prev = *e.get();
                            if dist + 1 < prev {
                                e.insert(dist + 1);
                                queue.push(((nx, ny), to));
                            }
                        }
                        Entry::Vacant(e) => {
                            e.insert(dist + 1);
                            queue.push(((nx, ny), to));
                        }
                    }
                }
            }
        }
    }

    let res = distances.values().max().unwrap();
    println!("{res}");

    let columns = grid[0].len() * 2;
    let rows = grid.len() * 2;
    let mut large_grid = vec![vec![b' '; columns]; rows];
    for y in 0..rows {
        for x in 0..columns {
            let xs = x / 2;
            let ys = y / 2;
            match (x % 2, y % 2) {
                (0, 0) => {
                    let pos = (xs as i32, ys as i32);
                    if distances.contains_key(&pos) {
                        large_grid[y][x] = grid[ys][xs];
                    }
                }
                (1, 0) => {
                    let x1 = (x - 1) / 2;
                    let x2 = (x + 1) / 2;
                    let Some(&a) = grid[ys].get(x1) else {
                        continue;
                    };
                    let Some(&b) = grid[ys].get(x2) else {
                        continue;
                    };

                    if distances.contains_key(&(x1 as i32, ys as i32))
                        && distances.contains_key(&(x2 as i32, ys as i32))
                        && (legal_move(a, b, 1, 0) || legal_move(b, a, -1, 0))
                    {
                        large_grid[y][x] = b'#';
                    } else {
                        large_grid[y][x] = b'.';
                    }
                }
                (0, 1) => {
                    let y1 = (y - 1) / 2;
                    let y2 = (y + 1) / 2;
                    let Some(&a) = grid.get(y1).and_then(|r| r.get(xs)) else {
                        continue;
                    };
                    let Some(&b) = grid.get(y2).and_then(|r| r.get(xs)) else {
                        continue;
                    };
                    if distances.contains_key(&(xs as i32, y1 as i32))
                        && distances.contains_key(&(xs as i32, y2 as i32))
                        && (legal_move(a, b, 0, 1) || legal_move(b, a, 0, -1))
                    {
                        large_grid[y][x] = b'#';
                    } else {
                        large_grid[y][x] = b'.';
                    }
                }
                (1, 1) => {
                    large_grid[y][x] = b'.';
                }
                _ => unreachable!(),
            }
        }
    }

    // Flood fill
    {
        let mut queue: Vec<Pos> = vec![
            (0, 0),
            (columns as i32 - 1, 0),
            (0, rows as i32 - 1),
            (columns as i32 - 1, rows as i32 - 1),
        ];
        for &(x, y) in &queue {
            large_grid[y as usize][x as usize] = b'o';
        }

        while !queue.is_empty() {
            let pos = queue.pop().unwrap();

            for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let nx = pos.0 + dx;
                let ny = pos.1 + dy;

                if let Some(&to) = large_grid
                    .get(ny as usize)
                    .and_then(|row| row.get(nx as usize))
                {
                    if to == b' ' || to == b'.' {
                        large_grid[ny as usize][nx as usize] = b'o';
                        queue.push((nx, ny));
                    }
                }
            }
        }

        let mut res2 = 0;
        for y in 0..rows {
            for x in 0..columns {
                print!("{}", large_grid[y][x] as char);
                if x % 2 == 0 && y % 2 == 0 && large_grid[y][x] == b' ' {
                    res2 += 1;
                }
            }
            println!();
        }
        println!();
        println!("{res2}");
    }
}

fn legal_move(from: u8, to: u8, dx: i32, dy: i32) -> bool {
    for (tile, (dx, dy)) in [(from, (dx, dy)), (to, (-dx, -dy))] {
        match (tile, (dx, dy)) {
            (b'S', (1, 0) | (0, 1) | (-1, 0) | (0, -1))
            | (b'|', (0, 1) | (0, -1))
            | (b'-', (1, 0) | (-1, 0))
            | (b'7', (-1, 0) | (0, 1))
            | (b'J', (-1, 0) | (0, -1))
            | (b'F', (1, 0) | (0, 1))
            | (b'L', (1, 0) | (0, -1)) => (),
            _ => return false,
        }
    }

    true
}
