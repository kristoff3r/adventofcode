fn main() {
    let input = include_str!("../in.txt");
    // let input = include_str!("../example.txt");

    let mut res = 0;
    let mut res2 = 0;
    for l in input.lines() {
        let mut values: Vec<i64> = l
            .split_ascii_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect();

        let mut chain = vec![values.clone()];
        while !values.iter().all(|&n| n == 0) {
            let tmp: Vec<i64> = values.windows(2).map(|p| p[1] - p[0]).collect();
            chain.push(tmp.clone());
            values = tmp;
        }

        let mut cur = 0;
        for row in chain.iter().rev() {
            cur += row.last().unwrap();
        }
        res += cur;

        let mut cur = 0;
        for row in chain.iter().rev() {
            cur = row.first().unwrap() - cur;
        }
        res2 += cur;
    }
    println!("{res}");
    println!("{res2}");
}
