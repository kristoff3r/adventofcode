fn main() {
    let input = std::fs::read_to_string("in.txt").unwrap();
    // let input = std::fs::read_to_string("example.txt").unwrap();

    let mut res1: i64 = 0;
    let mut res2: i64 = 0;
    for bank in input.lines() {
        res1 += calculate::<2>(bank);
        res2 += calculate::<12>(bank);
    }

    println!("{res1}");
    println!("{res2}");
}

fn calculate<const N: usize>(bank: &str) -> i64 {
    let mut maxs = [0; N];

    for (idx, battery) in bank.bytes().enumerate() {
        let battery = battery - b'0';

        let slots_left = bank.len() - idx;

        for i in 0..N {
            let remaining = N - i;
            if remaining <= slots_left && battery > maxs[i] {
                maxs[i] = battery;
                maxs[i + 1..].fill(0);
                break;
            }
        }
    }

    let res = maxs
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, b)| acc + i64::from(*b) * 10_i64.pow(i as u32));

    // println!("in={bank}");
    // println!("out={res}");

    res
}
