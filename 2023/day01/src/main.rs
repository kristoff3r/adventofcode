fn main() {
    let input = std::fs::read_to_string("in.txt").unwrap();

    let mut sum: i64 = 0;
    for l in input.lines() {
        let bl = l.as_bytes();
        let mut first_idx = l.find(|c: char| c.is_ascii_digit()).unwrap();
        let mut first = bl[first_idx] as char;
        let mut last_idx = l.rfind(|c: char| c.is_ascii_digit()).unwrap();
        let mut last = bl[last_idx] as char;
        for (i, p) in [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
        .iter()
        .enumerate()
        {
            let i = (i + 1) as u8;
            if let Some(tmp) = l.find(p) {
                if tmp < first_idx {
                    first_idx = tmp;
                    first = (i + '0' as u8) as char;
                }
            }
            if let Some(tmp) = l.rfind(p) {
                if tmp > last_idx {
                    last_idx = tmp;
                    last = (i + '0' as u8) as char;
                }
            }
        }
        let num = format!("{first}{last}");
        sum += num.parse::<i64>().unwrap();
    }

    println!("{sum}");
}
