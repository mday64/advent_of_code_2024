use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coord (i32, i32);
impl std::ops::Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl std::ops::AddAssign for Coord {

    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}
impl std::ops::Mul<i32> for Coord {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Coord(self.0 * rhs, self.1 * rhs)
    }
}

#[allow(dead_code)]
fn print_grid(grid: &HashMap<Coord, u8>) {
    for row in 0.. {
        if grid.get(&Coord(row, 0)).is_none() {
            break;
        }
        for col in 0.. {
            if let Some(ch) = grid.get(&Coord(row, col)) {
                print!("{}", *ch as char);
            } else {
                break;
            }
        }
        println!();
    }
}

pub fn part1(input: &str) -> i32 {
    let (warehouse, moves) = input.split_once("\n\n").expect("missing empyty line?");

    // Parse the map of the warehouse
    let mut grid = HashMap::new();
    let mut robot = None;
    for (row, line) in warehouse.lines().enumerate() {
        let row = row as i32;
        for (col, ch) in line.bytes().enumerate() {
            let col = col as i32;
            if ch == b'@' {
                robot = Some(Coord(row, col));
                grid.insert(Coord(row, col), b'.');
            } else {
                grid.insert(Coord(row, col), ch);
            }
        }
    }
    let mut robot = robot.expect("did not find robot's initial position");

    // print_grid(&grid);
    'outer: for dir in moves.bytes() {
        if dir == b'\n' { continue; }
        let dir = match dir {
            b'<' => Coord(0, -1),
            b'>' => Coord(0, 1),
            b'^' => Coord(-1, 0),
            b'v' => Coord(1, 0),
            _ => { panic!("Unknown move: {}", dir); }
        };

        // Try to move the robot and any boxes one space in the direction (d_row, d_col).
        // Look for the first empty space in that direction.
        let mut dest = robot;
        loop {
            dest += dir;
            match grid.get(&dest) {
                Some(&b'#') => continue 'outer,
                Some(&b'.') => break,
                _ => {}
            }
        }

        // If we got here, that means that everything between `dest` and `robot`
        // moves one step towards `dest`.  If there are spaces between `robot`
        // and `dest`, then they must all be boxes.
        robot += dir;
        if robot != dest {
            grid.insert(dest, b'O');
            grid.insert(robot, b'.');
        }
        // print_grid(&grid);
    }

    // Finally, compute the sum of coordinates of boxes
    grid.into_iter().filter_map(|(Coord(row, col), ch)| {
        if ch == b'O' {
            Some(100 * row + col)
        } else {
            None
        }
    }).sum()
}

//
// "Just like part 1, except everything (except the robot) is twice as wide"
//
// This means that pushing up or down can cause one box to push two boxes
// (if the one box overlaps both).  I think this means pushing N boxes in
// one row can push N+1 boxes in the next row.  Thankfully, if any one box
// is stuck by a wall, none of the boxes move.
//
// How am I going to represent the state of the warehouse?  If a given
// coordinate is occupied by (half of) a box, I need to know which coordinate
// contains the other half of the box.  I think I'm going to store a set of
// boxes, based on the coordinate of the left half of the box.  When checking
// for boxes in the way, be sure to check for both halves.
//
pub fn part2(input: &str) -> i32 {
    let (warehouse, moves) = input.split_once("\n\n").expect("missing empyty line?");

    // Parse the map of the warehouse
    let mut boxes = HashSet::new();     // Coord of left half of box
    let mut walls = HashSet::new();     // Coords of all wall squares
    let mut robot = None;
    for (line, row) in warehouse.lines().zip(0..) {
        for (ch, col) in line.bytes().zip(0..) {
            match ch {
                b'.' => {}
                b'@' => { robot = Some(Coord(row, 2 * col)); }
                b'O' => { boxes.insert(Coord(row, 2 * col)); }
                b'#' => {
                    walls.insert(Coord(row, 2 * col));
                    walls.insert(Coord(row, 2 * col + 1));
                }
                _ => panic!("unexpected character: {}", ch as char)
            }
        }
    }
    let mut robot = robot.expect("did not find robot's initial position");

    'next_move: for dir in moves.bytes() {
        if dir == b'\n' { continue; }

        match dir {
            b'<' => {
                let row = robot.0;
                let mut dest_col = robot.1 - 1;

                // Try to move the robot and any boxes one space to the
                // left.  Look for the first empty space in that direction.
                loop {
                    if walls.contains(&Coord(row, dest_col)) { continue 'next_move; }   // Can't move
                    if !boxes.contains(&Coord(row, dest_col-1)) { break; }
                    dest_col -= 2;
                }

                // If we got here, that means that everything between
                // `dest_col` and `robot` moves one step to the left.
                // If there are spaces between `robot` and `dest_col`,
                // then they must all be boxes.
                robot.1 -= 1;
                let mut temp = robot.1;
                while temp != dest_col {
                    assert!(boxes.remove(&Coord(row, temp-1)));
                    assert!(boxes.insert(Coord(row, temp-2)));
                    temp -= 2;
                }
            }
            b'>' => {
                let row = robot.0;
                let mut dest_col = robot.1 + 1;

                // Try to move the robot and any boxes one space to the
                // right.  Look for the first empty space in that direction.
                loop {
                    if walls.contains(&Coord(row, dest_col)) { continue 'next_move; }   // Can't move
                    if !boxes.contains(&Coord(row, dest_col)) { break; }
                    dest_col += 2;
                }

                // If we got here, that means that everything between
                // `dest_col` and `robot` moves one step to the right.
                // If there are spaces between `robot` and `dest_col`,
                // then they must all be boxes.
                robot.1 += 1;
                let mut temp = robot.1;
                while temp != dest_col {
                    assert!(boxes.remove(&Coord(row, temp)));
                    assert!(boxes.insert(Coord(row, temp+1)));
                    temp += 2;
                }
            }
            b'^' | b'v' => {
                // Moving vertically is very different.  Every box that moves can
                // push on two other boxes in the next row.  I think we need to go
                // row by row, building up a list of boxes to be pushed in that row,
                // until everything above/below is empty, or one or more walls.
                //
                // Also, selecting boxes for the row immediately below/above the
                // robot (i.e. directly pushed by the robot) is different from
                // other rows (where the boxes are pushed by other boxes).
                // That's because the robot is only one column wide, and can
                // only push in its own column, but boxes push in their indicated
                // column, and the column to the right.
                //
                // Hmm.  Should we maintain a set of columns being pushed
                // in the next row?  Note that we still need to check one
                // column to the left, too, in case we're pushing on the right
                // side of a box.
                let dir = if dir == b'^' { -1 } else { 1 };
                let mut boxes_to_move = HashSet::new();
                let mut row = robot.0;
                let mut cols_pushing = HashSet::from([robot.1]);

                while !cols_pushing.is_empty() {
                    row += dir;
                    let mut next_cols = HashSet::new();
                    for col in cols_pushing {
                        if walls.contains(&Coord(row, col)) {
                            // A wall is preventing us from moving
                            continue 'next_move;
                        }
                        if boxes.contains(&Coord(row, col)) {
                            boxes_to_move.insert(Coord(row, col));
                            next_cols.insert(col);
                            next_cols.insert(col+1);
                        }
                        if boxes.contains(&Coord(row, col-1)) {
                            boxes_to_move.insert(Coord(row, col-1));
                            next_cols.insert(col-1);
                            next_cols.insert(col);
                        }
                    }
                    cols_pushing = next_cols;
                }

                // If we get here, there were no boxes being pushed in `row`,
                // so we can go ahead and move all of the boxes_to_move one row.
                // Note that we remove them all first, then reinsert them all.
                // Otherwise, we'd have to do them in order from furthest row
                // to nearest row (just like an overlapping memcpy() in C).
                let dir = Coord(dir, 0);
                for b in boxes_to_move.iter() {
                    assert!(boxes.remove(b));
                }
                for b in boxes_to_move {
                    assert!(boxes.insert(b + dir));
                }
                robot += dir;
            }
            _ => panic!("Unknown direction: {}", dir)
        }
    }

    // Finally, compute the sum of coordinates of boxes
    boxes.into_iter().map(|Coord(row, col)| 100 * row + col).sum()
}

#[cfg(test)]
static FULL_INPUT: &str = include_str!("../input.txt");

#[test]
fn test_part1_tiny() {
    let input = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";
    assert_eq!(part1(input), 2028);
}

#[test]
fn test_part1_small() {
    let input = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";
    assert_eq!(part1(input), 10092);
}

#[test]
fn test_part1_full() {
    assert_eq!(part1(FULL_INPUT), 1552879);
}

#[test]
fn test_part2_tiny() {
    let input = "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";
    assert_eq!(part2(input), 105+207+306);
}

#[test]
fn test_part2_small() {
    let input = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";
    assert_eq!(part2(input), 9021);
}

#[test]
fn test_part2_full() {
    assert_eq!(part2(FULL_INPUT), 1561175);
}
