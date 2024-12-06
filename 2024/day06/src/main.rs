use std::{
    collections::HashSet,
    ops::{Index, IndexMut},
};

pub struct Grid<const M: usize, const N: usize> {
    pub grid: [[u8; M]; N],
}

impl<const M: usize, const N: usize> Grid<M, N> {
    pub fn from_file(path: &str) -> std::io::Result<Self> {
        let data = std::fs::read(path)?;

        let rows = data.split(|&b| b == b'\n').next().unwrap().len();
        let cols = data.split(|&b| b == b'\n').count() - 1;
        assert!(
            cols == M && rows == N,
            "Invalid dimensions, try {cols}x{rows}"
        );

        let mut grid = [[0; M]; N];
        for (i, row) in data.split(|&b| b == b'\n').enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                grid[i][j] = cell;
            }
        }

        for i in 0..M {
            for j in 0..N {
                assert!(grid[j][i] != 0, "{i},{j}");
            }
        }

        Ok(Self { grid })
    }

    pub fn print(&self) {
        for row in self.grid.iter() {
            println!("{}", String::from_utf8_lossy(row));
        }
    }

    pub fn find(&self, b: u8) -> Option<(i32, i32)> {
        for (i, row) in self.grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == b {
                    return Some((j as i32, i as i32));
                }
            }
        }

        None
    }

    pub fn get(&self, i: i32, j: i32) -> Option<u8> {
        self.grid
            .get(j as usize)
            .and_then(|row| row.get(i as usize))
            .cloned()
    }
}

impl<const M: usize, const N: usize> Index<(i32, i32)> for Grid<M, N> {
    type Output = u8;

    fn index(&self, (i, j): (i32, i32)) -> &u8 {
        &self.grid[j as usize][i as usize]
    }
}

impl<const M: usize, const N: usize> IndexMut<(i32, i32)> for Grid<M, N> {
    fn index_mut(&mut self, (i, j): (i32, i32)) -> &mut u8 {
        &mut self.grid[j as usize][i as usize]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn clockwise(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn step(self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

fn main() {
    let mut grid = Grid::<130, 130>::from_file("in.txt").unwrap();

    // let mut grid = Grid::<10, 10>::from_file("example.txt").unwrap();
    // grid.print();

    let start_pos = grid.find(b'^').unwrap();

    let mut visited = solve(&grid, start_pos).unwrap();
    println!("{}", visited.len());
    visited.remove(&start_pos);

    let mut res2 = 0;
    for pos in visited {
        grid[pos] = b'O';

        if solve(&grid, start_pos).is_none() {
            res2 += 1;
        }

        grid[pos] = b'.';
    }

    println!("{res2}");
}

fn solve<const M: usize, const N: usize>(
    grid: &Grid<M, N>,
    start_pos: (i32, i32),
) -> Option<HashSet<(i32, i32)>> {
    let mut direction = Direction::North;
    let mut cur_pos = start_pos;

    let mut visited = HashSet::new();
    visited.insert(cur_pos);

    let mut halted = false;

    for _ in 0..10000 {
        let step = direction.step();
        let (i, j) = (cur_pos.0 + step.0, cur_pos.1 + step.1);
        match grid.get(i, j) {
            Some(b'.' | b'X' | b'^') => {
                cur_pos = (i, j);
                visited.insert(cur_pos);
            }
            Some(b'#' | b'O') => {
                direction = direction.clockwise();
            }
            None => {
                halted = true;
                break;
            }
            Some(tile) => panic!("invalid tile: {tile}"),
        }

        if cur_pos == start_pos && direction == Direction::North {
            return None;
        }
    }

    if halted {
        Some(visited)
    } else {
        None
    }
}
