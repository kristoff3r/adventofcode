use std::collections::HashSet;

fn neighbors(x: usize, y: usize, max_len: usize) -> Vec<(usize, usize)> {
    let x = x as i32;
    let y = y as i32;
    let res = vec![
        (x - 1, y),
        (x, y - 1),
        (x - 1, y - 1),
        (x + 1, y),
        (x, y + 1),
        (x + 1, y + 1),
        (x + 1, y - 1),
        (x - 1, y + 1),
    ];

    res.into_iter()
        .filter(|&(x, y)| x >= 0 && y >= 0)
        .map(|(x, y)| (x as usize, y as usize))
        .filter(|&(x, y)| x < max_len && y < max_len)
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("in.txt").unwrap();

    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut last_digit = 0;
        let mut cur_number = String::new();
        let mut cur_indices = HashSet::new();
        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                if last_digit + 1 != x && !cur_number.is_empty() {
                    numbers.push((cur_number.parse::<i32>().unwrap(), cur_indices));
                    cur_indices = HashSet::new();
                    cur_number = String::new();
                }

                cur_indices.extend(neighbors(x, y, line.len()));
                cur_number.push(c);
                last_digit = x;
            } else if c != '.' {
                symbols.push(((x, y), c == '*'));
            }
        }
        if !cur_number.is_empty() {
            numbers.push((cur_number.parse::<i32>().unwrap(), cur_indices));
        }
    }

    let mut res1 = 0;
    for (sym, _is_gear) in &symbols {
        for (n, pos) in &numbers {
            if pos.contains(&sym) {
                res1 += n;
            }
        }
    }

    let mut res2 = 0;
    for (sym, is_gear) in &symbols {
        if !is_gear {
            continue;
        }
        let mut nums = Vec::new();
        for (n, pos) in &numbers {
            if pos.contains(&sym) {
                nums.push(*n);
            }
        }
        if nums.len() == 2 {
            res2 += nums.into_iter().product::<i32>();
        }
    }

    println!("{res1}");
    println!("{res2}");
}
