use rustc_hash::{FxHashMap, FxHashSet};

type Row = isize;
type Col = isize;
type Coord = (Row, Col);

//
// Oooh!  This is an interesting one!
//
// My plan is to build a mapping from row/col to number of steps to
// the end (assuming no cheats).  Then, for every location along the
// path, check locations two steps away, and see if they are on the
// path, and would result in fewer overall steps.
//
// My guess for part 2 is that we're allowed to pass through 2 walls,
// but they don't need to be adjacent.  That is, we get to cheat twice,
// one step each.
//
pub fn part1(input: &str) -> usize {
    part1_limit(input, 100)
}
pub fn part1_limit(input: &str, limit: usize) -> usize {
    let (start, end, track) = parse_input(input);
    let distances = solve_maze(start, end, track);

    //
    // And now we try cheating.
    //
    let mut result = 0;
    for (&(row, col), dist) in &distances {
        for neighbor in [
            (row-2, col), (row+2, col), (row, col-2), (row, col+2),
            (row-1, col-1), (row+1, col-1), (row-1, col-1), (row-1, col+1)
        ] {
            if let Some(&d) = distances.get(&neighbor) {
                if d >= dist + 2 + limit {
                    // eprintln!("start={:?} end={:?} saves {}", (row, col), neighbor, d - dist - 2);
                    result += 1;
                }
            }
        }
    }

    result
}

//
// My guess was wrong.  A "cheat" can be up to (and incuding) 20 steps.
//
pub fn part2(input: &str) -> usize {
    part2_limit(input, 100)
}
pub fn part2_limit(input: &str, limit: usize) -> usize {
    todo!()
}

//
// Parse the input string and return (start, end, track)
//
fn parse_input(input: &str) -> (Coord, Coord, FxHashSet<Coord>) {
    let mut track = FxHashSet::default();
    let mut start = None;
    let mut end = None;
    
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let row = row as isize;
            let col = col as isize;

            if ch == 'S' {
                start = Some((row, col));
            }
            if ch == 'E' {
                end = Some((row, col));
            }
            if ch != '#' {
                track.insert((row, col));
            }
        }
    }

    let start = start.expect("no start?");
    let end = end.expect("no end?");
    assert_ne!(start, end);

    (start, end, track)
}

fn solve_maze(start: Coord, end: Coord, track: FxHashSet<Coord>) -> FxHashMap<Coord, usize> {
    let mut distances = FxHashMap::default();
    let mut prev = (0, 0);      // this is a wall; didn't want to hassle with Option
    let mut curr = start;
    let mut dist = 0;
    distances.insert(start, 0);
    loop {
        // Take a step, but not back to `prev`
        for neighbor in [(curr.0-1, curr.1), (curr.0+1, curr.1), (curr.0, curr.1-1), (curr.0, curr.1+1)] {
            if neighbor != prev && track.contains(&neighbor) {
                prev = curr;
                curr = neighbor;
                break;
            }
        }
        dist += 1;
        distances.insert(curr, dist);

        if curr == end {
            break;
        }
    }

    distances
}

#[test]
fn test_part1() {
    let input = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";
    assert_eq!(part1_limit(input, 25), 4);
}

#[test]
fn test_part2() {
    let input = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";
    assert_eq!(part2_limit(input, 71), 22+4+3);
}

#[cfg(test)]
static FULL_INPUT: &str = include_str!("../input.txt");

#[test]
fn test_part1_full() {
    assert_eq!(part1(FULL_INPUT), 1485);
}

#[test]
fn test_part2_full() {
    assert_eq!(part2(FULL_INPUT), 1485);
}
