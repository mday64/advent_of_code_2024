use std::collections::HashMap;

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

pub fn part2(_input: &str) -> String {
    "World".to_string()
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
fn test_part2() {
    let input = "Hello, World!";
    assert_eq!(part2(input), "World");
}
