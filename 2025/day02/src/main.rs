use std::ops::RangeInclusive;

fn main() {
    let input = std::fs::read_to_string("in.txt").unwrap();
    // let input = std::fs::read_to_string("example.txt").unwrap();

    let mut res1 = 0;
    let mut res2 = 0;
    for range in input.trim().split(",") {
        let ids: RangeInclusive<i64> = range
            .split_once("-")
            .map(|(start, end)| start.parse().unwrap()..=end.parse().unwrap())
            .unwrap();
        res1 += ids.clone().filter(|&id| !is_valid(id)).sum::<i64>();
        res2 += ids.filter(|&id| !is_valid2(id)).map(|id| id).sum::<i64>();
    }

    println!("{res1}");
    println!("{res2}");
}

fn is_valid(id: i64) -> bool {
    let digits = 1 + id.ilog10();
    if digits % 2 != 0 {
        return true;
    }

    let n = 10_i64.pow(digits / 2);

    let part1 = id / n;
    let part2 = id % n;

    part1 != part2
}

fn is_valid2(id: i64) -> bool {
    let bytes = id.to_string().bytes().collect::<Vec<_>>();
    for n in 1..=bytes.len() / 2 {
        let mut iter = bytes[n..].chunks_exact(n);
        if iter.all(|c| c == &bytes[..n]) && iter.remainder().is_empty() {
            // println!("{id} invalid");
            return false;
        }
    }

    // println!("{id} valid");
    true
}
