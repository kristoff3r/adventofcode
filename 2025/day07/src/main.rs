use std::collections::{HashMap, HashSet, VecDeque};

use util::{Direction, Grid, glam::IVec2};

fn main() {
    let mut grid = Grid::<142, 141>::from_file("in.txt").unwrap();
    // let mut grid = Grid::<16, 15>::from_file("example.txt").unwrap();

    // grid.print();

    let start_pos = grid.find(b'S').unwrap();

    let mut queue = VecDeque::new();
    queue.push_back((start_pos, start_pos + Direction::South.step()));
    let mut tree: HashMap<IVec2, HashSet<IVec2>> = HashMap::new();

    while let Some((prev_pos, pos)) = queue.pop_front() {
        let Some(val) = grid.get(pos) else {
            continue;
        };

        match val {
            b'.' | b'|' => {
                grid[pos] = b'|';
                queue.push_back((prev_pos, pos + Direction::South.step()));
            }
            b'^' => {
                if tree.entry(prev_pos).or_default().insert(pos) {
                    queue.extend([
                        (pos, pos + Direction::East.step()),
                        (pos, pos + Direction::West.step()),
                    ]);
                }
            }
            _ => (),
        }
    }

    grid.print();

    // part 1 is the number of unique nodes we've seen
    let mut seen: HashSet<IVec2> = HashSet::new();
    for vals in tree.values() {
        seen.extend(vals);
    }
    println!("{}", seen.len());

    // part2 is the number of traversals
    let res2 = count_traversals(&tree, &mut HashMap::default(), start_pos);
    println!("{res2}");
}

fn count_traversals(
    tree: &HashMap<IVec2, HashSet<IVec2>>,
    memo: &mut HashMap<IVec2, i64>,
    pos: IVec2,
) -> i64 {
    if let Some(res) = memo.get(&pos).cloned() {
        return res;
    }
    let Some(children) = tree.get(&pos) else {
        return 1;
    };

    let mut res = 1;
    for child in children {
        res += count_traversals(tree, memo, *child);
    }

    memo.insert(pos, res);

    res
}
