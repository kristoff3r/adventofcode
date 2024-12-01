use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("in.txt").unwrap();

    let mut res = 0;
    let mut res2 = 0;
    for (id, game) in input.lines().enumerate() {
        let id = id + 1;
        let mut possible = true;
        let game = game.split(": ").nth(1).unwrap();
        let sets = game.split(";");
        let mut least_colors: HashMap<&str, i32> = HashMap::new();
        for s in sets {
            let s = s.split(", ");
            let mut colors: HashMap<_, i32> = [("red", 12), ("green", 13), ("blue", 14)]
                .into_iter()
                .collect();
            let mut cur_colors = HashMap::new();
            for p in s {
                let mut p = p.split_ascii_whitespace();
                let num: i32 = p.next().unwrap().parse().unwrap();
                let color = p.next().unwrap();
                let entry = colors.entry(color).or_default();
                if num > *entry {
                    possible = false;
                }
                *entry -= num;
                *cur_colors.entry(color).or_default() += num;
            }
            for (color, num) in cur_colors.iter() {
                let entry = least_colors.entry(color).or_default();
                *entry = (*entry).max(*num);
            }
        }
        res2 += least_colors.values().product::<i32>();
        if possible {
            res += id;
        }
    }

    println!("{res}");
    println!("{res2}");
}
