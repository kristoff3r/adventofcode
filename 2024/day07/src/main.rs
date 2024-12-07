use std::cmp::Ordering;

fn main() {
    let input = std::fs::read_to_string("in.txt").unwrap();
    // let input = std::fs::read_to_string("example.txt").unwrap();

    let mut res1 = 0;
    let mut res2: i64 = 0;
    for line in input.lines() {
        let mut parts = line.split(": ");
        let res = parts.next().unwrap().parse::<i64>().unwrap();
        let values = parts
            .next()
            .unwrap()
            .split(" ")
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        if solve(res, &values, &[add, mul]) {
            res1 += res;
            res2 += res;
            continue;
        }

        if solve(res, &values, &[add, mul, conc]) {
            res2 = res2.checked_add(res).unwrap();
            continue;
        }

        println!("failed {res}: {values:?}");
    }

    println!("{res1}");
    println!("{res2}");
}

fn add(a: i64, b: i64) -> i64 {
    a + b
}
fn mul(a: i64, b: i64) -> i64 {
    a * b
}
fn conc(a: i64, b: i64) -> i64 {
    format!("{a}{b}").parse().unwrap()
}

fn solve(res: i64, values: &[i64], operators: &[fn(i64, i64) -> i64]) -> bool {
    let mut queue = vec![(1, values[0], format!("{}", values[0]))];
    loop {
        let Some((idx, partial_res, ops)) = queue.pop() else {
            break;
        };

        for (i, op) in operators.iter().enumerate() {
            let Some(val) = values.get(idx) else {
                continue;
            };
            let new_res = op(partial_res, *val);
            let mut ops = ops.clone();
            ops.push_str(&format!(
                " {} {}",
                if i == 0 {
                    "+"
                } else if i == 1 {
                    "*"
                } else {
                    "||"
                },
                val
            ));
            //  println!("{ops} = {new_res}");
            match res.cmp(&new_res) {
                Ordering::Greater | Ordering::Equal if values.len() > idx + 1 => {
                    queue.push((idx + 1, new_res, ops));
                }
                Ordering::Equal => {
                    println!("found {ops} == {res}");
                    return true;
                }
                _ => {
                    // println!("{new_res} > {res}")
                }
            }
        }
    }

    false
}
