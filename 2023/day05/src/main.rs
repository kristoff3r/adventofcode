use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[derive(Debug)]
struct Mapping {
    source: i64,
    dest: i64,
    len: i64,
}

fn main() {
    let input = include_str!("../in.txt");
    // let input = include_str!("../example.txt");

    let (seeds, rest) = input.split_once('\n').unwrap();
    let seeds = seeds
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let (_, rest) = rest.split_once("\n").unwrap();

    let mut maps: Vec<Vec<Mapping>> = Vec::new();
    for map in rest.split("\n\n") {
        let (_name, mapping) = map.split_once("\n").unwrap();
        let mut map = Vec::new();
        for m in mapping.lines() {
            let m = m
                .split_ascii_whitespace()
                .map(|n| n.parse::<i64>().unwrap_or_else(|_| panic!("{n}")))
                .collect::<Vec<_>>();
            map.push(Mapping {
                dest: m[0],
                source: m[1],
                len: m[2],
            });
        }
        maps.push(map);
    }

    {
        let seeds = part1(maps.as_slice(), seeds.clone());
        println!("{}", seeds.iter().min().unwrap());
    }
    {
        let seed_ranges = seeds
            .chunks_exact(2)
            .map(|r| (r[0], r[0] + r[1]))
            .collect::<Vec<_>>();
        for r in &seed_ranges {
            let l = r.1 - r.0;
            println!("{l} {}", (l as f32).log2());
        }
        let res = seed_ranges
            .par_iter()
            .enumerate()
            .map(|(i, r)| {
                let mut res = i64::MAX;
                for mut s in r.0..r.1 {
                    for m in &maps {
                        for &Mapping { source, dest, len } in m {
                            if source <= s && s < source + len {
                                s = s - source + dest;
                                break;
                            }
                        }
                    }
                    res = res.min(s);
                }
                println!("Range {i}: {res}");
                res
            })
            .min()
            .unwrap();
        println!("Total: {res}");
    }
}

fn part1(maps: &[Vec<Mapping>], mut seeds: Vec<i64>) -> Vec<i64> {
    for m in maps {
        for s in seeds.iter_mut() {
            for &Mapping { source, dest, len } in m {
                if source <= *s && *s < source + len {
                    // println!("{s} -> {} because {cur:?}", *s - source + dest);
                    *s = *s - source + dest;
                    break;
                }
            }
        }
    }

    seeds
}

#[allow(dead_code)]
fn part2(maps: &[Vec<Mapping>], seeds: Vec<i64>) -> Vec<i64> {
    let mut seed_ranges = seeds
        .windows(2)
        .map(|r| (r[0], r[0] + r[1]))
        .collect::<Vec<_>>();
    for m in maps {
        let mut tmp = Vec::new();
        for (start, end) in seed_ranges.iter_mut() {
            let mut found = false;
            for &Mapping {
                source: source_start,
                dest,
                len,
            } in m
            {
                let source_end = source_start + len;
                let offset = dest - source_start;
                // No overlap
                if *start >= source_end || *end < source_start {
                    continue;
                }
                // Full overlap
                if source_start <= *start && *end <= source_end {
                    found = true;
                    tmp.push((*start + offset, *end + offset));
                    continue;
                }
                // Lower overlap
                if *start <= source_start {
                    let found_len = source_start - *end;
                    tmp.push((*start + offset, *end - found_len));
                    *start += offset;
                    continue;
                // Upper overlap
                } else {
                    let found_len = source_start - *end;
                    tmp.push((*start + offset, *end - found_len));
                    *end -= offset;
                }
            }

            if !found {
                tmp.push((*start, *end));
            }
        }
        seed_ranges = tmp;
    }

    seeds
}
