use std::ops::Range;

fn main() {
    let input = std::fs::read_to_string("in.txt").unwrap();
    // let input = std::fs::read_to_string("example.txt").unwrap();
    let mut iter = input.split("\n\n");

    let mut ranges: Vec<Range<i64>> = Vec::new();
    for range in iter.next().unwrap().lines() {
        let range = range
            .split_once("-")
            .map(|(start, end)| start.parse::<i64>().unwrap()..(end.parse::<i64>().unwrap() + 1))
            .unwrap();

        ranges.push(range);
    }

    ranges.sort_unstable_by(|a, b| a.start.cmp(&b.start));

    let mut merged_ranges = Vec::with_capacity(ranges.len());
    let mut cur_range = ranges[0].clone();
    for range in &ranges[1..] {
        if cur_range.end > range.start {
            cur_range = cur_range.start..range.end.max(cur_range.end);
        } else {
            merged_ranges.push(cur_range);
            cur_range = range.clone();
        }
    }
    merged_ranges.push(cur_range);

    println!("{}", merged_ranges.len());

    let mut res1 = 0;
    for id in iter.next().unwrap().lines() {
        let id = id.parse::<i64>().unwrap();
        for range in &merged_ranges {
            if range.contains(&id) {
                res1 += 1;
                break;
            }
        }
    }

    println!("{res1}");

    let res2: u64 = merged_ranges.into_iter().map(|r| r.count() as u64).sum();

    println!("{res2}");
}
