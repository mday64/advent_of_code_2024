mod part1 {
    use pathfinding::prelude::bfs_reach;
    use std::collections::HashMap;
    type Row = isize;
    type Col = isize;
    type Grid = HashMap<(Row, Col), u8>;
    pub fn part1(input: &str) -> usize {
        // Parse the input
        let mut grid: Grid = Grid::new();
        let mut zeroes = Vec::new();
        for (row, line) in input.lines().enumerate() {
            let row = row as isize;
            for (col, byte) in line.bytes().enumerate() {
                let col = col as isize;
                grid.insert((row, col), byte);
                if byte == b'0' { zeroes.push((row, col)) }
            }
        }

        // For each trailhead (b'0'), see how many unique trail ends (b'9')
        // are reachable.
        zeroes.into_iter().map(|(row, col)|
            bfs_reach(
                (row, col, &b'0'),
                |&(r, c, h)| {
                    [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
                        .into_iter()
                        .filter_map(|(rr, cc)| {
                            let h = *h;
                            if let Some(neighbor_height) = grid.get(&(rr,cc)) {
                                if *neighbor_height == h + 1 {
                                    Some((rr, cc, neighbor_height))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                }
            )
            .filter(|(_r, _c, h)| **h == b'9')
            .count()
        ).sum()
    }
}
pub use part1::part1;

pub fn part2(input: &str) -> usize {
    use pathfinding::prelude::count_paths;
    use std::collections::HashMap;
    type Row = isize;
    type Col = isize;
    type Grid = HashMap<(Row, Col), u8>;

    // Parse the input
    let mut grid: Grid = Grid::new();
    let mut zeroes = Vec::new();
    for (row, line) in input.lines().enumerate() {
        let row = row as isize;
        for (col, byte) in line.bytes().enumerate() {
            let col = col as isize;
            grid.insert((row, col), byte);
            if byte == b'0' {
                zeroes.push((row, col))
            }
        }
    }

    let mut result = 0;
    // For each trailhead (b'0'), see how many unique trail ends (b'9')
    // are reachable.
    for (row, col) in zeroes {
        result += count_paths(
            (row, col),
            |&(r, c)| {
                let height = grid.get(&(r, c)).unwrap();
                [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
                    .into_iter()
                    .filter(|&(rr, cc)| grid.get(&(rr, cc)) == Some(&(*height + 1)))
            },
            |loc| grid.get(loc) == Some(&b'9'),
        );
    }
    result
}

#[test]
fn test_part1_tiny() {
    let input = "\
0123
1234
8765
9876
";
    assert_eq!(part1(input), 1);
}

#[test]
fn test_part1_partial1() {
    let input = "\
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
";
    assert_eq!(part1(input), 4);
}

#[test]
fn test_part1_partial2() {
    let input = "\
10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01
";
    assert_eq!(part1(input), 3);
}

#[test]
fn test_part1_small() {
    let input = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
    assert_eq!(part1(input), 36);
}

#[test]
fn test_part2a() {
    let input = "\
.....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....
";
    assert_eq!(part2(input), 3);
}

#[test]
fn test_part2b() {
    let input = "\
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
";
    assert_eq!(part2(input), 13);
}

#[test]
fn test_part2c() {
    let input = "\
012345
123456
234567
345678
4.6789
56789.
";
    assert_eq!(part2(input), 227);
}

#[test]
fn test_part2d() {
    let input = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
    assert_eq!(part2(input), 81);
}

#[cfg(test)]
static FULL_INPUT: &str = include_str!("../input.txt");

#[test]
fn test_part1_full() {
    assert_eq!(part1(FULL_INPUT), 798);
}

#[test]
fn test_part2_full() {
    assert_eq!(part2(FULL_INPUT), 1816);
}
