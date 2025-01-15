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
    // fn reset(&mut self) {
    //     self.row = self.starting_row;
    //     self.col = self.starting_col;
    //     self.facing = Direction::Up;
    // }

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

mod both_parts {
    use super::Direction;
    use rustc_hash::{FxHashSet, FxHashMap};
    use ndarray::{Array, Dim, s};

    type Row = usize;
    type Col = usize;
    
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum GridSquare {
        Open,
        Obstacle,
        OutOfBounds
    }

    type Grid = Array<GridSquare, Dim<[usize; 2]>>;

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
        fn step(&mut self, obstacles: &Grid) -> GridSquare {
            let (row, col) = self.ahead();
            let kind = obstacles[[row, col]];
            match kind {
                GridSquare::Open => { self.row = row; self.col = col; }
                GridSquare::Obstacle => { self.facing.turn(); }
                GridSquare::OutOfBounds => {}
            }
            kind
        }
    
        // Take as many steps forward as possible.  If an obstacle is
        // reached, then turn instead.
        //
        // TODO: Use ndarray's own iteration rather than indexing.
        fn go_straight(&mut self, obstacles: &Grid) -> GridSquare {
            let mut kind: GridSquare;
            match self.facing {
                Direction::Up => {
                    kind = obstacles[[self.row - 1, self.col]];
                    while kind == GridSquare::Open {
                        self.row -= 1;
                        kind = obstacles[[self.row - 1, self.col]];
                    }
                }
                Direction::Right => {
                    kind = obstacles[[self.row, self.col + 1]];
                    while kind == GridSquare::Open {
                        self.col += 1;
                        kind = obstacles[[self.row, self.col + 1]];
                    }
                }
                Direction::Down => {
                    kind = obstacles[[self.row + 1, self.col]];
                    while kind == GridSquare::Open {
                        self.row += 1;
                        kind = obstacles[[self.row + 1, self.col]];
                    }
                }
                Direction::Left => {
                    kind = obstacles[[self.row, self.col - 1]];
                    while kind == GridSquare::Open {
                        self.col -= 1;
                        kind = obstacles[[self.row, self.col - 1]];
                    }
                }
            };

            if kind == GridSquare::Obstacle {
                self.facing.turn();
            }

            kind
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

    pub fn both_parts(input: &str) -> (usize, usize) {
        let num_rows = input.lines().count();
        let num_cols = input.lines().next().unwrap().len();

        // Build a 2D array from the input, with extra rows and columns
        // on each edge set to OutOfBounds.
        let mut grid = Array::from_elem((num_rows+2, num_cols+2), GridSquare::OutOfBounds);
        let mut guard: Option<(usize, usize)> = None;
        for (row, line) in input.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                grid[[row+1, col+1]] = match ch {
                    '.' => GridSquare::Open,
                    '#' => GridSquare::Obstacle,
                    '^' => {
                        guard = Some((row+1, col+1));
                        GridSquare::Open
                    }
                    other => panic!("Invalid input: {}", other)
                };
            }
        }

        // Initialize the guard to the position found in the input.
        let guard = guard.unwrap();
        let mut guard = Guard::new(guard.0, guard.1);

        // Part 1: Count how many unique positions the guard occupies
        // before traveling out of bounds.
        //
        // The value (Direction) is the direction the guard was facing when
        // they first reached the given position.
        let mut visited = FxHashMap::<(Row, Col), Direction>::default();
        loop {
            visited.entry(guard.current_position()).or_insert(guard.facing);
            if guard.step(&grid) == GridSquare::OutOfBounds {
                break;
            }
        }
        let part1 = visited.len();

        // Part 2: Count how many grid squares could be changed to Obstacle
        // (one at a time!) to force the guard to get into a loop.
        // Note that the guard's initial position is not allowed.
        let mut part2 = 0;
        visited.remove(&(guard.starting_row, guard.starting_col));
        for ((row, col), facing) in visited {
            grid[[row, col]] = GridSquare::Obstacle;
            guard.reset_before(facing, row, col);
            let mut visited = FxHashSet::<(Row, Col, Direction)>::default();
            loop {
                let kind = guard.go_straight(&grid);
                if kind == GridSquare::OutOfBounds {
                    break;
                }
                if !visited.insert((guard.row, guard.col, guard.facing)) {
                    part2 += 1;
                    break;
                }
            }
            grid[[row, col]] = GridSquare::Open;
        }

        (part1, part2)
    }
}

pub use both_parts::both_parts;

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

#[test]
fn test_both_parts() {
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
    assert_eq!(both_parts(input), (41, 6));
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

#[test]
fn test_both_parts_full() {
    assert_eq!(both_parts(INPUT), (5461, 1836));
}
