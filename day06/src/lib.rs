use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up, Right, Down, Left
}

impl Direction {
    fn turn(&mut self) {
        *self = match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up
        }
    }
}

type Row = i16;
type Col = i16;
struct Guard {
    row: Row,
    col: Col,
    facing: Direction,
    starting_row: Row,
    starting_col: Col,
}

impl Guard {
    fn new(row: Row, col: Col) -> Guard {
        Guard { row, col, facing: Direction::Up, starting_row: row, starting_col: col }
    }

    fn current_position(&self) -> (Row, Col) {
        (self.row, self.col)
    }

    fn ahead(&self) -> (Row, Col) {
        match self.facing {
            Direction::Up => (self.row - 1, self.col),
            Direction::Right => (self.row, self.col + 1),
            Direction::Down => (self.row + 1, self.col),
            Direction::Left => (self.row, self.col - 1)
        }
    }

    // Try to make one step forward.  If there is an obstacle there,
    // then turn instead.
    fn step(&mut self, obstacles: &HashSet<(Row, Col)>) {
        let ahead = self.ahead();
        if obstacles.contains(&ahead) {
            self.facing.turn();
        } else {
            self.row = ahead.0;
            self.col = ahead.1;
        }
    }

    // Reset the guard to it's starting position and direction
    fn reset(&mut self) {
        self.row = self.starting_row;
        self.col = self.starting_col;
        self.facing = Direction::Up;
    }
}

pub fn part1(input: &str) -> usize {
    let mut obstacles = HashSet::<(Row, Col)>::new();
    let mut guard = (0, 0);
    let num_rows = input.lines().count() as Row;
    let num_cols = input.lines().next().unwrap().len() as Col;
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            match ch {
                '.' => {}
                '#' => { obstacles.insert((row as Row, col as Col)); }
                '^' => { guard = (row as Row, col as Col); }
                other => panic!("Bad input: {}", other)
            }
        }
    }
    let mut guard = Guard::new(guard.0, guard.1);

    let mut visited = HashSet::<(Row, Col)>::new();
    while (0..num_rows).contains(&guard.row) && (0..num_cols).contains(&guard.col) {
        visited.insert(guard.current_position());
        guard.step(&obstacles);
    }

    visited.len()
}

//
// Count how many locations for a single new obstacle that would cause
// the guard to get into a loop.
//
// The brute force solution is to try every possible location that is
// not an obstacle, and not the guard's initial position, and then
// see whether the guard exits the grid, or returns to a prior location
// and direction.
//
// We can do at least a little better.  The new obstacle needs to cause
// the guard to turn towards another obstacle.  It has to be one of the
// locations the locations they visited in part 1 (i.e. on their path,
// and in their way, causing them to make a new turn).  It has to be
// one row/column away from a another obstacle.
//
// For example, if the guard was moving left, the new obstacle needs
// to be one column to the left of an obstacle above the guard's path
// (since the obstacle will cause them to turn to the right, from Left
// to Up).  So, if we keep track of both the position and direction they
// were facing before moving into that position, we only need to search
// in one direction for a pre-existing obstacle.  I think we still need
// to simulate their path with the new obstacle (or just the new path
// after the new turn?) to make sure they are in fact in a loop (and
// won't exit the grid in a new location).
//
pub fn part2(input: &str) -> u32 {
    let mut obstacles = HashSet::<(Row, Col)>::new();
    let mut guard = (0, 0);
    let num_rows = input.lines().count() as Row;
    let num_cols = input.lines().next().unwrap().len() as Col;
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            match ch {
                '.' => {}
                '#' => { obstacles.insert((row as Row, col as Col)); }
                '^' => { guard = (row as Row, col as Col); }
                other => panic!("Bad input: {}", other)
            }
        }
    }
    let mut guard = Guard::new(guard.0, guard.1);

    let mut visited = HashSet::<(Row, Col)>::new();
    while (0..num_rows).contains(&guard.row) && (0..num_cols).contains(&guard.col) {
        visited.insert(guard.current_position());
        guard.step(&obstacles);
    }

    // Try putting an obstacle at each of the visited positions, reset
    // the guard to that position, and see whether they get into a loop.
    visited.remove(&(guard.starting_row, guard.starting_col));
    let mut result = 0;
    for (row, col) in visited.iter() {
        obstacles.insert((*row, *col));
        let mut visited = HashSet::<(Direction, (Row, Col))>::new();
        guard.reset();
        while (0..num_rows).contains(&guard.row) && (0..num_cols).contains(&guard.col) {
            if !visited.insert((guard.facing, guard.current_position())) {
                // Found a loop
                // dbg!((row, col));
                result += 1;
                break;
            }
            guard.step(&obstacles);
        }
        obstacles.remove(&(*row, *col));
    }

    result
}

#[test]
fn test_part1() {
    let input = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
    assert_eq!(part1(input), 41);
}

#[test]
fn test_part2() {
    let input = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
    assert_eq!(part2(input), 6);
}

#[cfg(test)]
const INPUT: &str = include_str!("../input.txt");

#[test]
fn test_part1_full() {
    assert_eq!(part1(INPUT), 5461);
}

#[test]
fn test_part2_full() {
    assert_eq!(part2(INPUT), 1836);
}
