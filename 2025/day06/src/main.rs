fn main() {
    let input = std::fs::read_to_string("in.txt").unwrap();
    // let input = std::fs::read_to_string("example.txt").unwrap();

    let lines = input.lines().collect::<Vec<_>>();
    let (operations, numbers) = lines.split_last().unwrap();
    let numbers = numbers
        .iter()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut res1: i64 = 0;
    for (i, op) in operations.split_ascii_whitespace().enumerate() {
        let iter = numbers.iter().map(|nums| nums[i]);
        match op {
            "+" => res1 += iter.sum::<i64>(),
            "*" => res1 += iter.product::<i64>(),
            _ => (),
        }
    }

    println!("{res1}");

    let max_len = lines.iter().map(|l| l.len()).max().unwrap();
    let mut numbers = Vec::new();
    let mut cur_numbers = Vec::new();
    for idx in 0..max_len {
        let mut num = String::new();
        for line in &lines {
            let Some(c) = line.chars().nth(idx) else {
                continue;
            };
            if c.is_ascii_digit() {
                num.push(c);
            }
        }

        if let Ok(num) = num.parse::<i64>() {
            cur_numbers.push(num);
        } else {
            numbers.push(cur_numbers);
            cur_numbers = Vec::new();
        }
    }

    if !cur_numbers.is_empty() {
        numbers.push(cur_numbers);
    }

    let mut res2 = 0;
    for (i, op) in operations.split_ascii_whitespace().enumerate() {
        match op {
            "+" => res2 += &numbers[i].iter().sum::<i64>(),
            "*" => res2 += &numbers[i].iter().product::<i64>(),
            _ => (),
        }
    }

    println!("{res2}");
}
