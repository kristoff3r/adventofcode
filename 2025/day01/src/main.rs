fn main() {
    let input = std::fs::read_to_string("in.txt").unwrap();
    // let input = std::fs::read_to_string("example.txt").unwrap();

    let mut current = 50;
    let mut res1 = 0;
    let mut res2 = 0;
    for line in input.lines() {
        let (dir, amount) = line.split_at(1);
        let amount: i32 = amount.parse().unwrap();

        let was_zero = current == 0;

        match dir {
            "L" => current -= amount,
            "R" => current += amount,
            _ => {
                unreachable!()
            }
        }

        if current <= 0 || current >= 100 {
            let add = match current.signum() {
                0 => 1,
                1 => current / 100,
                -1 => (if was_zero { 0 } else { 1 }) - current / 100,
                _ => unreachable!(),
            };
            res2 += add;
        }

        current = current.rem_euclid(100);
        if current == 0 {
            res1 += 1;
        }
    }

    println!("{res1}");
    println!("{res2}");
}
