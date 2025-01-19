use std::collections::{HashMap, HashSet};
use itertools::Itertools;

struct Position(i32, i32);


pub fn part1(input: &str) -> usize {
    let num_rows = input.lines().count() as i32;
    let num_cols = input.lines().next().unwrap().len() as i32;
    let mut antennas = HashMap::<char, Vec<Position>>::new();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch != '.' {
                antennas.entry(ch).or_default().push(Position(row as i32, col as i32));
            }
        }
    }

    let mut results = HashSet::new();
    
    for (_ch, positions) in antennas.iter() {
        for pair in positions.iter().combinations(2) {
            let d_row = pair[1].0 - pair[0].0;
            let d_col = pair[1].1 - pair[0].1;

            let t_row = pair[1].0 + d_row;
            let t_col = pair[1].1 + d_col;
            if t_row >= 0 && t_row < num_rows && t_col >= 0 && t_col < num_cols {
                // dbg!((t_row, t_col));
                results.insert((t_row, t_col));
            }

            let t_row = pair[0].0 - d_row;
            let t_col = pair[0].1 - d_col;
            if t_row >= 0 && t_row < num_rows && t_col >= 0 && t_col < num_cols {
                // dbg!((t_row, t_col));
                results.insert((t_row, t_col));
            }
        }
    }

    results.len()
}

pub fn part2(input: &str) -> usize {
    let num_rows = input.lines().count() as i32;
    let num_cols = input.lines().next().unwrap().len() as i32;
    let mut antennas = HashMap::<char, Vec<Position>>::new();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch != '.' {
                antennas.entry(ch).or_default().push(Position(row as i32, col as i32));
            }
        }
    }

    let mut results = HashSet::new();
    
    for (_ch, positions) in antennas.iter() {
        for pair in positions.iter().combinations(2) {
            let d_row = pair[1].0 - pair[0].0;
            let d_col = pair[1].1 - pair[0].1;

            let mut row = pair[0].0;
            let mut col = pair[0].1;
            loop {
                row += d_row;
                col += d_col;
                if row >= 0 && row < num_rows && col >= 0 && col < num_cols {
                    // dbg!((row, col));
                    results.insert((row, col));
                } else {
                    break;
                }
            }

            row = pair[1].0;
            col = pair[1].1;
            loop {
                row -= d_row;
                col -= d_col;
                if row >= 0 && row < num_rows && col >= 0 && col < num_cols {
                    // dbg!((row, col));
                    results.insert((row, col));
                } else {
                    break;
                }
            }
        }
    }

    results.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_2() {
        let input = "\
..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........
";
        assert_eq!(part1(input), 2);
    }
    
    #[test]
    fn test_part1_4() {
        let input = "\
..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
..........
";
        assert_eq!(part1(input), 4);
    }
    
    #[test]
    fn test_part1_14() {
        let input = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
        assert_eq!(part1(input), 14);
    }
    
    #[test]
    fn test_part2_9() {
        let input = "\
T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........
";
        assert_eq!(part2(input), 9);
    }
    
    #[test]
    fn test_part2_34() {
        let input = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
        assert_eq!(part2(input), 34);
    }
}
