fn main() {
    let input = include_str!("../in.txt");

    let times = input
        .lines()
        .nth(0)
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let distances = input
        .lines()
        .nth(1)
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut res = Vec::new();
    for (&t, &d) in times.iter().zip(distances.iter()) {
        let mut cur_res = 0;
        for i in 0..t {
            let cur_d = i * (t - i);
            if cur_d > d {
                cur_res += 1;
            }
        }
        res.push(cur_res);
    }

    println!("{}", res.iter().product::<i64>());

    let t = 46689866;
    let d: i128 = 358105418071080;
    let mut cur_res = 0;
    for i in 0..t {
        let cur_d = i * (t - i);
        if cur_d > d {
            cur_res += 1;
        }
    }
    println!("{cur_res}");
}

// t = h_t + r_t
// x = h_t*r_t
