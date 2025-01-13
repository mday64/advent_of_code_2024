use std::collections::HashSet;

#[derive(Debug)]
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

struct Guard<'a> {
    row: isize,
    col: isize,
    obstacles: &'a HashSet<(isize, isize)>,
    facing: Direction
}

impl<'a> Guard<'a> {
    fn new(row: isize, col: isize, obstacles: &'a HashSet<(isize, isize)>) -> Guard<'a> {
        Guard { row, col, obstacles, facing: Direction::Up }
    }

    fn current_position(&self) -> (isize, isize) {
        (self.row, self.col)
    }

    fn ahead(&self) -> (isize, isize) {
        match self.facing {
            Direction::Up => (self.row - 1, self.col),
            Direction::Right => (self.row, self.col + 1),
            Direction::Down => (self.row + 1, self.col),
            Direction::Left => (self.row, self.col - 1)
        }
    }

    fn step(&mut self) {
        let ahead = self.ahead();
        if self.obstacles.contains(&ahead) {
            self.facing.turn();
        } else {
            self.row = ahead.0;
            self.col = ahead.1;
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut obstacles = HashSet::<(isize, isize)>::new();
    let mut guard = (0, 0);
    let num_rows = input.lines().count() as isize;
    let num_cols = input.lines().next().unwrap().len() as isize;
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            match ch {
                '.' => {}
                '#' => { obstacles.insert((row as isize, col as isize)); }
                '^' => { guard = (row as isize, col as isize); }
                other => panic!("Bad input: {}", other)
            }
        }
    }
    let mut guard = Guard::new(guard.0, guard.1, &obstacles);

    let mut visited = HashSet::<(isize, isize)>::new();
    while (0..num_rows).contains(&guard.row) && (0..num_cols).contains(&guard.col) {
        visited.insert(guard.current_position());
        guard.step();
    }

    visited.len()
}

pub fn part2(_input: &str) -> u32 {
    43
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
    let input = "Hello, World!";
    assert_eq!(part2(input), 43);
}
