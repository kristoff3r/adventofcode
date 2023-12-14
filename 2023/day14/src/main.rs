use std::collections::HashMap;

fn main() {
    let input = include_str!("../in.txt").as_bytes();
    // let input = include_str!("../example.txt").as_bytes();

    let mut grid: Vec<_> = input
        .split(|&b| b == b'\n')
        .filter(|r| !r.is_empty())
        .map(|r| r.to_vec())
        .collect();

    let height = grid.len();
    let width = grid[0].len();

    {
        let mut grid = grid.clone();
        for y in 1..height {
            for x in 0..width {
                if grid[y][x] == b'O' {
                    let mut swap_pos = y;
                    loop {
                        let Some(&b) = grid.get(swap_pos - 1).and_then(|r| r.get(x)) else {
                            break;
                        };
                        if b != b'.' {
                            break;
                        }
                        swap_pos -= 1;
                        if swap_pos == 0 {
                            break;
                        }
                    }
                    if swap_pos != y {
                        let (p1, p2) = grid.split_at_mut(swap_pos + 1);
                        std::mem::swap(&mut p1[swap_pos][x], &mut p2[y - swap_pos - 1][x]);
                    }
                }
            }
        }

        let mut res = 0;
        for (y, row) in grid.iter().enumerate() {
            res += row.iter().filter(|&&b| b == b'O').count() * (height - y);
        }
        println!("{res}");
    }

    let mut seen: HashMap<Vec<Vec<u8>>, Vec<usize>> = HashMap::new();
    // 1000000000 == 154+84*x + y
    // 1000000000
    for i in 0..1000000000 {
        if i == 160 {
            let mut res = 0;
            for (y, row) in grid.iter().enumerate() {
                res += row.iter().filter(|&&b| b == b'O').count() * (height - y);
            }
            println!("{i:06}: {res}");
            break;
        }
        if i % 1000 == 0 {
            println!("{:.2}%", 100.0 * i as f32 / 1000000000.0);
            for entries in seen.values() {
                println!(
                    "{entries:?} {:?}",
                    entries.windows(2).map(|x| x[1] - x[0]).collect::<Vec<_>>()
                );
            }
        }
        let mut res = 0;
        for (y, row) in grid.iter().enumerate() {
            res += row.iter().filter(|&&b| b == b'O').count() * (height - y);
        }
        println!("{i:06}: {res}");
        spin_grid(&mut grid);
        seen.entry(grid.clone()).or_default().push(i);
        // for (y, row) in grid.iter().enumerate() {
        //     for (x, &b) in row.iter().enumerate() {
        //         print!("{}", b as char);
        //     }
        //     println!();
        // }
        // println!();
        // println!();
        // println!();
        // println!();
    }

    let mut res = 0;
    for (y, row) in grid.iter().enumerate() {
        res += row.iter().filter(|&&b| b == b'O').count() * (height - y);
    }
    println!("{res}");
}

fn spin_grid(grid: &mut Vec<Vec<u8>>) {
    let height = grid.len();
    let width = grid[0].len();

    // North
    for y in 1..height {
        for x in 0..width {
            if grid[y][x] == b'O' {
                let mut swap_pos = y;
                loop {
                    let Some(&b) = grid.get(swap_pos - 1).and_then(|r| r.get(x)) else {
                        break;
                    };
                    if b != b'.' {
                        break;
                    }
                    swap_pos -= 1;
                    if swap_pos == 0 {
                        break;
                    }
                }
                if swap_pos != y {
                    let (p1, p2) = grid.split_at_mut(swap_pos + 1);
                    std::mem::swap(&mut p1[swap_pos][x], &mut p2[y - swap_pos - 1][x]);
                }
            }
        }
    }

    // West
    for y in 0..height {
        for x in 1..width {
            if grid[y][x] == b'O' {
                let mut swap_pos = x;
                loop {
                    let Some(&b) = grid.get(y).and_then(|r| r.get(swap_pos - 1)) else {
                        break;
                    };
                    if b != b'.' {
                        break;
                    }
                    swap_pos -= 1;
                    if swap_pos == 0 {
                        break;
                    }
                }
                if swap_pos != x {
                    let row = &mut grid[y];
                    let (p1, p2) = row.split_at_mut(swap_pos + 1);
                    std::mem::swap(&mut p1[swap_pos], &mut p2[x - swap_pos - 1]);
                }
            }
        }
    }

    // South
    for y in (0..height - 1).rev() {
        for x in 0..width {
            if grid[y][x] == b'O' {
                let mut swap_pos = y;
                loop {
                    let Some(&b) = grid.get(swap_pos + 1).and_then(|r| r.get(x)) else {
                        break;
                    };
                    if b != b'.' {
                        break;
                    }
                    swap_pos += 1;
                }
                if swap_pos != y {
                    let (p1, p2) = grid.split_at_mut(y + 1);
                    std::mem::swap(&mut p1[y][x], &mut p2[swap_pos - y - 1][x]);
                }
            }
        }
    }

    // East
    for y in 0..height {
        for x in (0..width - 1).rev() {
            if grid[y][x] == b'O' {
                let mut swap_pos = x;
                loop {
                    let Some(&b) = grid.get(y).and_then(|r| r.get(swap_pos + 1)) else {
                        break;
                    };
                    if b != b'.' {
                        break;
                    }
                    swap_pos += 1;
                }
                if swap_pos != x {
                    let row = &mut grid[y];
                    let (p1, p2) = row.split_at_mut(x + 1);
                    std::mem::swap(&mut p1[x], &mut p2[swap_pos - x - 1]);
                }
            }
        }
    }
}
