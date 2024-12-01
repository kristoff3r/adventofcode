use std::convert::identity;

fn main() {
    let input = std::fs::read("in.txt").unwrap();
    // let input = include_bytes!("../example.txt");

    let grid: Vec<_> = input
        .split(|&b| b == b'\n')
        .filter(|r| !r.is_empty())
        .map(|l| l.to_owned())
        .collect();

    let height = grid.len();
    let width = grid[0].len();

    let mut to_insert_x = Vec::new();
    let mut to_insert_y = Vec::new();

    for x in 0..width {
        if (0..height).map(|y| grid[y][x] == b'.').all(identity) {
            to_insert_x.push(x);
        }
    }
    for (y, row) in grid.iter().enumerate() {
        if row.iter().all(|&c| c == b'.') {
            to_insert_y.push(y);
        }
    }

    let mut galaxies = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == b'#' {
                let xdiff = to_insert_x.iter().take_while(|&&i| i < x).count() * (1000000 - 1);
                let ydiff = to_insert_y.iter().take_while(|&&i| i < y).count() * (1000000 - 1);
                galaxies.push((x + xdiff, y + ydiff));
            }
            print!("{}", grid[y][x] as char);
        }
        println!();
    }

    let mut res = 0;
    for (i, &(x1, y1)) in galaxies.iter().enumerate() {
        for &(x2, y2) in &galaxies[i..] {
            res += x2.abs_diff(x1) + y2.abs_diff(y1);
        }
    }
    println!("{res}");
}
