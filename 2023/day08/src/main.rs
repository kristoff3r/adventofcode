use std::collections::HashMap;

fn main() {
    let input = include_str!("../in.txt");

    let mut lines = input.lines();
    let directions = lines.next().unwrap();
    lines.next();

    let mut nodes = HashMap::new();
    let mut queue = Vec::new();
    for l in lines {
        let (dest, rest) = l.split_once(" = (").unwrap();
        nodes.insert(dest, (&rest[0..3], &rest[5..8]));
        if dest.ends_with('A') {
            queue.push((dest, 0));
        }
    }

    {
        let mut cur_node = "AAA";
        let mut steps = 0;
        for d in directions.chars().cycle() {
            let choice = nodes[cur_node];
            match d {
                'L' => {
                    cur_node = choice.0;
                }
                'R' => {
                    cur_node = choice.1;
                }
                _ => panic!("invalid direction {d}"),
            }
            steps += 1;
            if cur_node == "ZZZ" {
                break;
            }
        }
        println!("{steps}");
    }

    let mut done = Vec::new();
    for d in directions.chars().cycle() {
        for (node, cycle) in queue.iter_mut() {
            let choice = nodes[node];
            match d {
                'L' => {
                    *node = choice.0;
                }
                'R' => {
                    *node = choice.1;
                }
                _ => panic!("invalid direction {d}"),
            }
            *cycle += 1;
        }
        queue.retain(|(n, c)| {
            if n.ends_with('Z') {
                done.push(*c as i64);
                false
            } else {
                true
            }
        });

        if queue.is_empty() {
            break;
        }
    }

    let res = done.into_iter().fold(1, num::integer::lcm);
    println!("{res}");
}
