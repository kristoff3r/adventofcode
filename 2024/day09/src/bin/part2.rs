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
        .filter_map(|(i, blocks)| {
            if blocks == 0 {
                return None;
            }

            let block = if i % 2 == 0 {
                let block = Block::File {
                    id: i / 2,
                    file_blocks: blocks,
                };
                block
            } else {
                Block::Empty {
                    empty_blocks: blocks,
                }
            };

            Some(block)
        })
        .collect::<Vec<_>>();

    // print_disk(&disk);

    for id in (0..10000).rev() {
        let i = disk
            .iter()
            .rposition(|block| block.file_id() == Some(id))
            .unwrap();

        let file_blocks = disk[i].blocks();
        for j in 0..i {
            if !disk[j].is_empty() {
                continue;
            }
            if file_blocks <= disk[j].blocks() {
                let diff = disk[j].blocks() - file_blocks;
                disk.swap(i, j);
                if diff > 0 {
                    disk[i].decrease(diff);
                    assert!(disk[i].is_empty());
                    assert!(disk[i].blocks() > 0);
                    disk.insert(j + 1, Block::Empty { empty_blocks: diff });
                }
                // print_disk(&disk);
                break;
            }
        }

        for i in 0..disk.len() {
            if i >= disk.len() {
                break;
            }
            if disk[i].is_empty() {
                while i + 1 < disk.len() && disk[i + 1].is_empty() {
                    let blocks = disk[i + 1].blocks();
                    disk[i].increase(blocks);
                    disk.remove(i + 1);
                }
            }
        }
    }

    let mut res = 0;
    let mut idx = 0;
    for block in &disk {
        for i in idx..idx + block.blocks() {
            res += i * block.file_id().unwrap_or_default();
        }
        idx += block.blocks();
    }

    print_disk(&disk);
    println!("{disk:#?}");
    println!("{res}");
}

fn print_disk(disk: &[Block]) {
    for &block in disk {
        match block {
            Block::File { id, file_blocks } => {
                print!("{}", format!("{}", (id % 10)).as_str().repeat(file_blocks));
            }
            Block::Empty { empty_blocks } => {
                print!("{}", ".".repeat(empty_blocks));
            }
        }
    }

    println!();
}

#[derive(Copy, Clone, Debug)]
enum Block {
    File { id: usize, file_blocks: usize },
    Empty { empty_blocks: usize },
}

impl Block {
    pub fn blocks(self) -> usize {
        match self {
            Block::File { file_blocks, .. } => file_blocks,
            Block::Empty { empty_blocks } => empty_blocks,
        }
    }

    pub fn file_id(self) -> Option<usize> {
        if let Block::File { id, .. } = self {
            Some(id)
        } else {
            None
        }
    }

    pub fn is_empty(self) -> bool {
        matches!(self, Block::Empty { .. })
    }

    pub fn decrease(&mut self, n: usize) {
        match self {
            Block::File { file_blocks, .. } => *file_blocks -= n,
            Block::Empty { empty_blocks } => *empty_blocks -= n,
        }
    }

    pub fn increase(&mut self, n: usize) {
        match self {
            Block::File { file_blocks, .. } => *file_blocks += n,
            Block::Empty { empty_blocks } => *empty_blocks += n,
        }
    }
}
