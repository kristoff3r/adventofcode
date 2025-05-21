fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut elves: Vec<u64> = input
        .split("\n\n")
        .map(|elf| {
            elf.split_whitespace()
                .map(|item| item.parse::<u64>().unwrap())
                .sum()
        })
        .collect();
    let part1 = elves.iter().max().unwrap();
    println!("{part1}");

    elves.sort();
    let part2: u64 = elves.iter().rev().take(3).sum();
    println!("{part2}");
}
