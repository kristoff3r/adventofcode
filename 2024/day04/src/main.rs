fn check(xs: impl Iterator<Item = u8>) -> bool {
    let w = xs.take(4).collect::<Vec<_>>();

    &w == b"XMAS" || &w == b"SAMX"
}

fn main() {
    let input = std::fs::read("in.txt").unwrap();
    // let input = std::fs::read("example.txt").unwrap();

    let grid: Vec<_> = input
        .split(|&b| b == b'\n')
        .filter(|r| !r.is_empty())
        .map(|l| l.to_owned())
        .collect();

    let mut debug = vec![vec![b'.'; grid[0].len()]; grid.len()];
    let mut res1 = 0;

    let lookup = |i, j, f: fn(usize, usize, usize) -> (usize, usize)| {
        (0..4)
            .map(move |k| f(i, j, k))
            .flat_map(|(i, j)| grid.get(i).and_then(|xs| xs.get(j)))
            .copied()
    };

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            for dir in [
                lookup(i, j, |i, j, k| (i, j + k)),
                lookup(i, j, |i, j, k| (i + k, j)),
                lookup(i, j, |i, j, k| (i + k, j + k)),
                lookup(i, j, |i, j, k| (i + k, j - k)),
            ] {
                if check(dir) {
                    res1 += 1;
                }
            }
        }
    }

    println!("{res1}");

    let mut res2 = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == b'A' {
                let corners = [
                    grid.get(i - 1).and_then(|xs| xs.get(j - 1)),
                    grid.get(i + 1).and_then(|xs| xs.get(j - 1)),
                    grid.get(i - 1).and_then(|xs| xs.get(j + 1)),
                    grid.get(i + 1).and_then(|xs| xs.get(j + 1)),
                ]
                .into_iter()
                .flatten()
                .filter(|&&b| b == b'M' || b == b'S')
                .collect::<Vec<_>>();

                if corners.len() == 4 && corners[0] != corners[3] && corners[1] != corners[2] {
                    debug[i][j] = grid[i][j];
                    debug[i + 1][j + 1] = grid[i + 1][j + 1];
                    debug[i - 1][j + 1] = grid[i - 1][j + 1];
                    debug[i + 1][j - 1] = grid[i + 1][j - 1];
                    debug[i - 1][j - 1] = grid[i - 1][j - 1];
                    res2 += 1;
                }
            }
        }
    }

    // for row in debug {
    //     println!("{}", String::from_utf8_lossy(&row));
    // }

    println!("{res2}");
}
