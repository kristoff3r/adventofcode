use std::iter;

fn main() {
    let mut disk = std::fs::read_to_string("in.txt")
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .map(|&n| (n - b'0') as usize)
        .enumerate()
        .flat_map(|(i, blocks)| {
            iter::repeat_n(
                if i % 2 == 0 {
                    Block::File { id: i / 2 }
                } else {
                    Block::Empty
                },
                blocks,
            )
        })
        .collect::<Vec<_>>();

    let mut start = 0;
    let mut end = disk.len() - 1;

    while start != end {
        let (part1, part2) = disk.split_at_mut(end);
        if part2[0].is_empty() {
            end -= 1;
            continue;
        };

        if !part1[start].is_empty() {
            start += 1;
            continue;
        };
        std::mem::swap(&mut part1[start], &mut part2[0]);

        // print_disk(&disk);
    }

    let mut res1 = 0;
    for (idx, block) in disk.iter().enumerate() {
        res1 += idx * block.file_id().unwrap_or_default();
    }

    print_disk(&disk);
    println!("{res1}");
}

fn print_disk(disk: &[Block]) {
    let mut out = String::new();
    for &block in disk {
        match block {
            Block::File { id } => {
                out.push_str(&format!("{}", (id % 10)));
            }
            Block::Empty => {
                out.push('.');
            }
        }
    }

    println!("{out}");
}

#[derive(Copy, Clone, Debug)]
enum Block {
    File { id: usize },
    Empty,
}

impl Block {
    pub fn file_id(self) -> Option<usize> {
        if let Block::File { id } = self {
            Some(id)
        } else {
            None
        }
    }

    pub fn is_empty(self) -> bool {
        matches!(self, Block::Empty)
    }
}
