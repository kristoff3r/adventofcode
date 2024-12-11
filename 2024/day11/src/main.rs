use std::{collections::HashMap, fmt::format};

fn main() {
    let input = std::fs::read_to_string("in.txt").unwrap();
    // let input = std::fs::read_to_string("example.txt").unwrap();

    let input = input
        .lines()
        .next()
        .unwrap()
        .split(" ")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut counts = HashMap::new();

    counts.extend(input.into_iter().map(|n| (n, 1)));

    // for _ in 0..25 {
    for _ in 0..75 {
        let mut next = HashMap::new();
        for (n, count) in counts.drain() {
            let digits = n.checked_ilog10().unwrap_or(0) + 1;
            let res = match n {
                0 => &[1][..],
                n if digits % 2 == 0 => {
                    let val = 10_i64.pow(digits / 2);
                    &[n / val, n % val][..]
                }
                n => &[n * 2024][..],
            };
            for n in res.into_iter().copied() {
                *next.entry(n).or_default() += count;
            }
        }
        counts = next;
        // println!("{counts:?}");
    }

    let res: i64 = counts.values().sum();
    println!("{res}");
}
