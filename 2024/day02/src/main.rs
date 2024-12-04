fn check(mut seq: impl Iterator<Item = i64> + Clone) -> bool {
    seq.clone().all(|x| 1 <= x && x <= 3) || seq.all(|x| -3 <= x && x <= -1)
}

fn main() {
    let input = std::fs::read_to_string("in.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    let mut res1 = 0;
    let mut res2 = 0;
    for report in input {
        let seq = report.windows(2).map(|x| x[0] - x[1]);
        if check(seq.clone()) {
            res1 += 1;
        }

        for i in 0..report.len() {
            let seq = report
                .iter()
                .enumerate()
                .filter(|&(j, _)| j != i)
                .map(|(_, x)| x)
                .collect::<Vec<_>>();
            let seq = seq.windows(2).map(|x| x[0] - x[1]);

            if check(seq) {
                res2 += 1;
                break;
            }
        }
    }

    println!("{res1}");
    println!("{res2}");
}
