mod part1 {
    use pathfinding::prelude::dfs;
    use std::collections::HashMap;
    type Row = isize;
    type Col = isize;
    type Grid = HashMap<(Row, Col), u8>;
    pub fn part1(input: &str) -> usize {
        // Parse the input
        let mut grid: Grid = Grid::new();
        let mut zeroes = Vec::new();
        let mut nines = Vec::new();
        for (row, line) in input.lines().enumerate() {
            let row = row as isize;
            for (col, byte) in line.bytes().enumerate() {
                let col = col as isize;
                grid.insert((row, col), byte);
                match byte {
                    b'0' => zeroes.push((row, col)),
                    b'9' => nines.push((row, col)),
                    _ => {}
                }
            }
        }

        let mut result = 0;
        // For each trailhead (b'0'), see how many unique trail ends (b'9')
        // are reachable.
        for (row, col) in zeroes {
            for dest in nines.iter() {
                if dfs(
                    (row, col),
                    |&(r, c)| {
                        let height = grid.get(&(r, c)).unwrap();
                        [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
                            .into_iter()
                            .filter(|&(rr, cc)| {
                                grid.get(&(rr, cc)) == Some(&(*height + 1))
                            })
                    },
                    |loc| loc == dest,
                )
                .is_some()
                {
                    // println!("({}, {}) => ({}, {})", row, col, dest.0, dest.1);
                    result += 1;
                }
            }
        }
        result
    }
}
pub use part1::part1;

pub fn part2(_input: &str) -> String {
    "World".to_string()
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
fn test_part2() {
    let input = "Hello, World!";
    assert_eq!(part2(input), "World");
}
