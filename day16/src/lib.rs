use rustc_hash::{FxHashMap, FxHashSet};
use pathfinding::prelude::{astar, astar_bag};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    East,
    North,
    West,
    South    
}
impl Direction {
    fn clockwise(&self) -> Self {
        match self {
            Self::East => Self::South,
            Self::North => Self::East,
            Self::West => Self::North,
            Self::South => Self::West
        }
    }

    fn counterclockwise(&self) -> Self {
        match self {
            Self::East => Self::North,
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut grid = FxHashMap::default();
    let mut start = None;
    let mut end = None;
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.bytes().enumerate() {
            match ch {
                b'S' => {
                    start = Some((row, col));
                    grid.insert((row, col), b'.');
                }
                b'E' => {
                    end = Some((row, col));
                    grid.insert((row, col), b'.');
                }
                _ => {
                    grid.insert((row, col), ch);
                }
            }
        }
    }
    let start = (start.expect("no start?"), Direction::East);
    let end = end.expect("no end?");

    // Note the ": &_" bit is to work around a compiler error.
    // See https://github.com/rust-lang/rust/issues/70263
    let success = |&(location, _direction): &_| { location == end };
    let heuristic = |&((row, col), _direction): &_| {
        end.0.abs_diff(row) + end.1.abs_diff(col)
    };
    let successors = |&((row, col), direction) : &((usize, usize), Direction)| {
        // produce something that can turn into an iterator of (Node, Cost)
        let mut result = vec![
            (((row, col), direction.clockwise()), 1000),
            (((row, col), direction.counterclockwise()), 1000)
        ];

        let forward = match direction {
            Direction::East => (row, col+1),
            Direction::North => (row-1, col),
            Direction::West => (row, col-1),
            Direction::South => (row+1, col),
        };
        if grid.get(&forward) == Some(&b'.') {
            result.push(((forward, direction), 1));
        }

        result
    };
    let (_path, cost) = astar(&start, successors, heuristic, success).expect("no solution found");
    cost
}

pub fn part2(input: &str) -> usize {
    let mut grid = FxHashMap::default();
    let mut start = None;
    let mut end = None;
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.bytes().enumerate() {
            match ch {
                b'S' => {
                    start = Some((row, col));
                    grid.insert((row, col), b'.');
                }
                b'E' => {
                    end = Some((row, col));
                    grid.insert((row, col), b'.');
                }
                _ => {
                    grid.insert((row, col), ch);
                }
            }
        }
    }
    let start = (start.expect("no start?"), Direction::East);
    let end = end.expect("no end?");

    // Note the ": &_" bit is to work around a compiler error.
    // See https://github.com/rust-lang/rust/issues/70263
    let success = |&(location, _direction): &_| { location == end };
    let heuristic = |&((row, col), _direction): &_| {
        end.0.abs_diff(row) + end.1.abs_diff(col)
    };
    let successors = |&((row, col), direction) : &((usize, usize), Direction)| {
        // produce something that can turn into an iterator of (Node, Cost)
        let mut result = vec![
            (((row, col), direction.clockwise()), 1000),
            (((row, col), direction.counterclockwise()), 1000)
        ];

        let forward = match direction {
            Direction::East => (row, col+1),
            Direction::North => (row-1, col),
            Direction::West => (row, col-1),
            Direction::South => (row+1, col),
        };
        if grid.get(&forward) == Some(&b'.') {
            result.push(((forward, direction), 1));
        }

        result
    };
    let (paths, _cost) = astar_bag(&start, successors, heuristic, success).expect("no solution found");
    
    let mut locations = FxHashSet::default();
    for path in paths {
        for (location, _direction) in path {
            locations.insert(location);
        }
    }

    locations.len()
}

#[test]
fn test_part1_example_1() {
    let input = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";
    assert_eq!(part1(input), 7036);
}

#[test]
fn test_part1_example_2() {
    let input = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";
    assert_eq!(part1(input), 11048);
}

#[test]
fn test_part2_example_1() {
    let input = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";
    assert_eq!(part2(input), 45);
}

#[test]
fn test_part2_example_2() {
    let input = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";
    assert_eq!(part2(input), 64);
}

#[cfg(test)]
static FULL_INPUT: &str = include_str!("../input.txt");

#[test]
fn test_part1_full() {
    assert_eq!(part1(FULL_INPUT), 66404);
}

#[test]
fn test_part2_full() {
    assert_eq!(part2(FULL_INPUT), 433);
}
