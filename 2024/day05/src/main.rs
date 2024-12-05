use std::collections::HashMap;

use petgraph::{algo::toposort, prelude::DiGraphMap};

fn main() {
    let input = std::fs::read_to_string("in.txt").unwrap();
    // let input = std::fs::read_to_string("example.txt").unwrap();

    let parts = input.split("\n\n").collect::<Vec<_>>();

    let edges = parts[0].lines().map(|rule| {
        let (a, b) = rule.split_at(2);
        let a = a.parse::<i64>().unwrap();
        let b = b[1..].parse::<i64>().unwrap();

        (a, b)
    });
    let graph = DiGraphMap::<_, ()>::from_edges(edges.clone());

    let mut res1 = 0;
    let mut res2 = 0;
    for update in parts[1].lines() {
        let iter = update.split(",").map(|n| n.parse::<i64>().unwrap());
        let valid = iter
            .clone()
            .zip(iter.clone().skip(1))
            .all(|(a, b)| graph.contains_edge(a, b));
        if valid {
            let len = (iter.clone().count() - 1) / 2;
            let middle = iter.clone().skip(len).next().unwrap();
            res1 += middle;
            // println!("valid {middle} {:?}", iter.collect::<Vec<_>>());
        } else {
            let mut update = iter.collect::<Vec<_>>();
            let graph = DiGraphMap::<_, ()>::from_edges(
                edges
                    .clone()
                    .filter(|(a, b)| update.contains(a) && update.contains(b)),
            );
            let sorted_nodes = toposort(&graph, None).unwrap();
            let lookup: HashMap<i64, usize> = sorted_nodes
                .into_iter()
                .enumerate()
                .map(|(a, b)| (b, a))
                .collect();

            update.sort_by(|a, b| lookup[a].cmp(&lookup[b]));

            let iter = update.iter();
            for (a, b) in iter.clone().zip(iter.clone().skip(1)) {
                if !graph.contains_edge(*a, *b) {
                    println!("{a} -> {b} invalid");
                }
            }

            let middle = update[(update.len() - 1) / 2];
            // println!("{update:?} {middle}");
            res2 += middle;
        }
    }

    println!("{res1}");
    println!("{res2}");
}
