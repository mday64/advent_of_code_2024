use rustc_hash::FxHashMap;
use pathfinding::prelude::astar;

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

    let success = |&(location, _direction)| { location == end };
    let heuristic = |&((row, col), _direction)| {
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

pub fn part2(_input: &str) -> String {
    "World".to_string()
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
fn test_part2() {
    let input = "Hello, World!";
    assert_eq!(part2(input), "World");
}
