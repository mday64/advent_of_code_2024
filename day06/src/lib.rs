use rustc_hash::{FxHashMap, FxHashSet};

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
    fn step(&mut self, obstacles: &FxHashSet<(Row, Col)>) {
        let ahead = self.ahead();
        if obstacles.contains(&ahead) {
            self.facing.turn();
        } else {
            self.row = ahead.0;
            self.col = ahead.1;
        }
    }

    // Reset the guard to its starting position and direction
    fn reset(&mut self) {
        self.row = self.starting_row;
        self.col = self.starting_col;
        self.facing = Direction::Up;
    }

    // Reset the guard to its position right before moving to a given
    // location from the given direction.
    fn reset_before(&mut self, facing: Direction, row: Row, col: Col) {
        self.facing = facing;
        self.row = row;
        self.col = col;
        // Take a step backwards
        match facing {
            Direction::Up => { self.row += 1; }
            Direction::Right => { self.col -= 1; }
            Direction::Down => { self.row -= 1; }
            Direction::Left => {self.col += 1; }
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut obstacles = FxHashSet::<(Row, Col)>::default();
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

    let mut visited = FxHashSet::<(Row, Col)>::default();
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
pub fn part2(input: &str) -> usize {
    let mut obstacles = FxHashSet::<(Row, Col)>::default();
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

    // The value (Direction) is the direction the guard was facing when
    // they first reached the given position.
    let mut visited = FxHashMap::<(Row, Col), Direction>::default();
    while (0..num_rows).contains(&guard.row) && (0..num_cols).contains(&guard.col) {
        visited.entry(guard.current_position()).or_insert(guard.facing);
        guard.step(&obstacles);
    }

    // Try putting an obstacle at each of the visited positions, reset
    // the guard to that position, and see whether they get into a loop.
    // Note: we need to remove the guard's initial position.
    visited.remove(&(guard.starting_row, guard.starting_col));
    let mut new_obstacles = FxHashSet::<(Row, Col)>::default();
    for ((row, col), facing) in visited.iter() {
        obstacles.insert((*row, *col));
        let mut visited = FxHashSet::<(Direction, (Row, Col))>::default();
        guard.reset_before(*facing, *row, *col);
        while (0..num_rows).contains(&guard.row) && (0..num_cols).contains(&guard.col) {
            if !visited.insert((guard.facing, guard.current_position())) {
                // Found a loop
                // dbg!((row, col));
                new_obstacles.insert((*row, *col));
                break;
            }
            guard.step(&obstacles);
        }
        obstacles.remove(&(*row, *col));
    }

    new_obstacles.len()
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
