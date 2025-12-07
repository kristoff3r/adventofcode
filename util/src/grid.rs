use std::ops::{Index, IndexMut};

use glam::{ivec2, IVec2};

pub struct Grid<const M: usize, const N: usize> {
    pub grid: [[u8; N]; M],
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

        let mut grid = [[0; N]; M];
        for (i, row) in data.split(|&b| b == b'\n').enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                grid[i][j] = cell;
            }
        }

        for i in 0..M {
            for j in 0..N {
                assert!(grid[i][j] != 0, "{i},{j}");
            }
        }

        Ok(Self { grid })
    }

    pub fn print(&self) {
        for row in self.grid.iter() {
            println!("{}", String::from_utf8_lossy(row));
        }
    }

    pub fn find(&self, b: u8) -> Option<IVec2> {
        for (i, row) in self.grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == b {
                    return Some(ivec2(j as i32, i as i32));
                }
            }
        }

        None
    }

    pub fn in_bounds(&self, pos: IVec2) -> bool {
        0 <= pos.x && pos.x < N as i32 && 0 <= pos.y && pos.y < M as i32
    }

    pub fn get(&self, pos: IVec2) -> Option<u8> {
        self.grid
            .get(pos.y as usize)
            .and_then(|row| row.get(pos.x as usize))
            .cloned()
    }

    pub fn iter_all(&self) -> impl Iterator<Item = (IVec2, u8)> + Clone + use<'_, M, N> {
        self.grid
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .copied()
                    .enumerate()
                    .map(move |(j, c)| (ivec2(i as i32, j as i32), c))
            })
            .flatten()
    }

    pub fn neighbors(&self, pos: IVec2) -> impl Iterator<Item = (IVec2, u8)> + use<'_, M, N> {
        [
            pos + Direction::North.step(),
            pos + Direction::East.step(),
            pos + Direction::South.step(),
            pos + Direction::West.step(),
        ]
        .into_iter()
        .filter_map(|pos| self.get(pos).map(|v| (pos, v)))
    }

    pub fn neighbors_8(&self, pos: IVec2) -> impl Iterator<Item = (IVec2, u8)> + use<'_, M, N> {
        [
            pos + Direction::North.step(),
            pos + Direction::North.step() + Direction::East.step(),
            pos + Direction::North.step() + Direction::West.step(),
            pos + Direction::South.step(),
            pos + Direction::South.step() + Direction::East.step(),
            pos + Direction::South.step() + Direction::West.step(),
            pos + Direction::East.step(),
            pos + Direction::West.step(),
        ]
        .into_iter()
        .filter_map(|pos| self.get(pos).map(|v| (pos, v)))
    }
}

impl<const M: usize, const N: usize> Index<IVec2> for Grid<M, N> {
    type Output = u8;

    fn index(&self, p: IVec2) -> &u8 {
        &self.grid[p.y as usize][p.x as usize]
    }
}

impl<const M: usize, const N: usize> IndexMut<IVec2> for Grid<M, N> {
    fn index_mut(&mut self, p: IVec2) -> &mut u8 {
        &mut self.grid[p.y as usize][p.x as usize]
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

    pub fn step(self) -> IVec2 {
        match self {
            Direction::North => ivec2(0, -1),
            Direction::East => ivec2(1, 0),
            Direction::South => ivec2(0, 1),
            Direction::West => ivec2(-1, 0),
        }
    }
}
