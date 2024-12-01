fn main() {
    let input = std::fs::read_to_string("in.txt").unwrap();

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in input.lines() {
        let numbers = &line
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        list1.push(numbers[0]);
        list2.push(numbers[1]);
    }
    list1.sort();
    list2.sort();

    let res = list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<u64>();
    println!("{}", res);

    let res = list1
        .iter()
        .map(|n| n * list2.iter().filter(|&m| n == m).count() as u64)
        .sum::<u64>();
    println!("{}", res);
}
