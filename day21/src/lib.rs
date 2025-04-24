use std::{collections::HashMap, sync::LazyLock};
use itertools::Itertools;

//
// There are three robots here.  Number one is typing on the numeric
// keypad (for the codes we are given).  Number two is typing on the
// directional keypad of number one.  Number three is typing on the
// directional keypad of number two.  We (the human) need to type
// on the directional keypad of number three.
//
// NOTE: Any movement between keys consists of zero or more up-or-down
// moves (but not both up and down), and zero or more left-or-right
// moves (but not both left and right).  It makes sense to do all of
// the up-or-down moves in a row (pressing the A key multiple times).
// The same goes for left-or-right.
//
// NOTE: Each keypad has one "missing" square that a robot may not
// hover over.  This makes certain orders of movement (left-right, then
// up-down, or vice versa) preferable.  For example, on the numeric
// keypad, going from 0/A to 1/4/7 should be up then left; going the
// other way should be right then down.  On the directional keypad
// (starting from A), you should go down before left, and right before
// up (to get to or from "<"; otherwise, it doesn't matter).
//
// QUESTION: For the numeric keypad, when staying within a rectangle,
// and it doesn't matter whether you go up-down or left-right first,
// does it save steps to use the directional keypad's preferred order?
// For example, moving from 3 to 7 on the numeric keypad (two left
// and two up), it is more efficient for robot two to move left to "^",
// press it two times, move down and left to "<" and press it two times.
// The alternative would be to move down once and left twice (in either
// order) to get to the "<" button, press it twice, and then move right
// and up to "^", and press it twice.  I think that means that (if
// possible) you should move up or right, then down, then left.
// Conventiently, if the numeric keypad needs to go up and right, it
// will never hit the "missing" square, and it doesn't matter which
// one is first.
//
//
// NOTE: The moves of the human pressing the directional pad of robot
// three don't have a cost, and don't have constraints.  For simplicity
// (and in anticipation of part 2 having a longer chain of robots), the
// human should move in the same way that a robot would.
//
// Note: We could either create two different kinds of robots (depending
// on the type of keypad they're pushing buttons on), or a single class
// that knows the layout of both kinds of keypads (and has the logic
// for avoiding the missing square).
//
// NOTE: Every code ends with "A".  Every robot's last action is to
// press the "A" key on the keypad in front of it.  So, for every code,
// every robot starts and ends by hovering over "A" on the keypad in
// front of it.  Therefore, we don't need to maintain state between
// codes.
//
// NOTE: If we have a long chain of robots (part 2?), it would probably
// make sense to precompute or cache the resulting presses given a
// starting symbol and a destination symbol.  Would it actually improve
// the performance of part 1?  Or would the cost of caching be worse
// than the repeated calculations?
//
// The keypads:
// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
//
//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
//

pub fn part1(input: &str) -> usize {
    input.lines().map(|line| {
        let code = line.strip_suffix('A').unwrap().parse::<usize>().unwrap();
        let robot1 = presses_for_numeric_code(line);
        let robot2 = presses_for_directional_code(&robot1);
        let robot3 = presses_for_directional_code(&robot2);
        // println!("{line}\n{robot1}\n{robot2}\n{robot3}\n");
        code * robot3.len()
    }).sum()
}

//
// Part 2 is the same as part 1, but with 25 robots typing on directional
// keypads (instead of 2 for part 1).
//
fn part2_inner(input: &str, depth: u32) -> u64 {
    let mut cache = HashMap::new();
    input.lines().map(|line| {
        let code = line.strip_suffix('A').unwrap().parse::<u64>().unwrap();
        let seq = format!("A{line}");
        code * seq.chars()
            .tuple_windows()
            .map(|(a,b)| num_presses_for_code(a, b, true, depth, &mut cache))
            .sum::<u64>()
    }).sum()
}
pub fn part2(input: &str) -> u64 {
    part2_inner(input, 25)
}

//
// Return the ways to get from the `src` key to the `dest` key, and
// press `dest`.  The result is one or more strings containing
// directional keys.
//
fn ways_for_keys(src: char, dest: char, numeric: bool) -> Vec<String> {
    let keypad = if numeric { &NUMERIC_KEYPAD } else { &DIRECTIONAL_KEYPAD };
    let &(src_row, src_col) = keypad.get(&src).unwrap();
    let &(dest_row, dest_col) = keypad.get(&dest).unwrap();

    // Zig-zag is always suboptimal.  The only ways we care about are
    // either vertical then horizontal, or horizontal then vertical.
    let mut horizontal = String::new();
    let mut vertical = String::new();
    if dest_col > src_col {
        for _ in src_col..dest_col {
            horizontal.push('>');
        }
    } else if dest_col < src_col {
        for _ in dest_col..src_col {
            horizontal.push('<');
        }
    }
    if dest_row > src_row {
        for _ in src_row..dest_row {
            vertical.push('v');
        }
    } else if dest_row < src_row {
        for _ in dest_row..src_row {
            vertical.push('^');
        }
    }

    // We need to avoid moving over the missing spot on the keyboard.
    if numeric {
        if src_row == 3 && dest_col == 0 {
            // Must go up, then left
            vec![format!("{vertical}{horizontal}A")]
        } else if src_col == 0 && dest_row == 3 {
            // Must go right, then down
            vec![format!("{horizontal}{vertical}A")]
        } else if src_row == dest_row || src_col == dest_col {
            // horizontal and/or vertical are empty, so there is only
            // one combination
            vec![format!("{vertical}{horizontal}A")]
        } else {
            vec![format!("{vertical}{horizontal}A"), format!("{horizontal}{vertical}A")]
        }
    } else {
        if src_row == 0 && dest_col == 0 {
            // Must go down, then left
            vec![format!("{vertical}{horizontal}A")]
        } else if src_col == 0 && dest_row == 0 {
            // Must go right, then up
            vec![format!("{horizontal}{vertical}A")]
        } else if src_row == dest_row || src_col == dest_col {
            // horizontal and/or vertical are empty, so there is only
            // one combination
            vec![format!("{vertical}{horizontal}A")]
        } else {
            vec![format!("{vertical}{horizontal}A"), format!("{horizontal}{vertical}A")]
        }
    }
}

#[test]
fn test_ways_for_keys_numeric() {
    assert_eq!(ways_for_keys('A', '4', true), ["^^<<A"]);
    assert_eq!(ways_for_keys('7', '0', true), [">vvvA"]);
    assert_eq!(ways_for_keys('A', '0', true), ["<A"]);
    assert_eq!(ways_for_keys('0', '2', true), ["^A"]);
    assert_eq!(ways_for_keys('2', '9', true), ["^^>A", ">^^A"]);
    assert_eq!(ways_for_keys('9', 'A', true), ["vvvA"]);
    assert_eq!(ways_for_keys('A', '9', true), ["^^^A"]);
    assert_eq!(ways_for_keys('4', '6', true), [">>A"]);
    assert_eq!(ways_for_keys('6', '4', true), ["<<A"]);
}

#[test]
fn test_ways_for_keys_directional() {
    assert_eq!(ways_for_keys('A', '<', false), ["v<<A"]);
    assert_eq!(ways_for_keys('<', 'A', false), [">>^A"]);
    assert_eq!(ways_for_keys('<', '^', false), [">^A"]);
    assert_eq!(ways_for_keys('^', '<', false), ["v<A"]);
    assert_eq!(ways_for_keys('A', 'v', false), ["v<A", "<vA"]);
    assert_eq!(ways_for_keys('v', 'A', false), ["^>A", ">^A"]);
    assert_eq!(ways_for_keys('^', '>', false), ["v>A", ">vA"]);
    assert_eq!(ways_for_keys('>', '^', false), ["^<A", "<^A"]);
    assert_eq!(ways_for_keys('<', '>', false), [">>A"]);
    assert_eq!(ways_for_keys('>', '<', false), ["<<A"]);
    assert_eq!(ways_for_keys('A', '>', false), ["vA"]);
    assert_eq!(ways_for_keys('>', 'A', false), ["^A"]);
}

//
// Return the number of key presses needed to go from key `src` to key
// `dest`, and press `dest`.  `depth` is the number of remaining robots
// in the chain.
//
fn num_presses_for_code(
    src: char,
    dest: char,
    numeric: bool,
    depth: u32,
    cache: &mut HashMap<(char, char, bool, u32), u64>)
    -> u64
{
    if let Some(&result) = cache.get(&(src, dest, numeric, depth)) {
        return result;
    }

    if depth == 0 {
        assert_eq!(numeric, false);
        let result = ways_for_keys(src, dest, numeric).into_iter()
            .map(|s| s.len())
            .min()
            .unwrap()
            as u64;
        cache.insert((src, dest, numeric, depth), result);
        return result;
    }

    let result = ways_for_keys(src, dest, numeric).into_iter()
        .map(|seq| {
            let seq = format!("A{seq}");

            seq.chars()
                .tuple_windows()
                .map(|(a, b)| num_presses_for_code(a, b, false, depth-1, cache))
                .sum()
        })
        .min()
        .unwrap();

    cache.insert((src, dest, numeric, depth), result);

    result
}

//
// Given a code that a robot must enter on the keypad in front of it
// (eg., "029A" or "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"),
// return the key presses needed on this robot's own directional keypad.
//
// Assumes that the robot's arm is currently hovering over the "A" key.
//
type Row = i32;
type Col = i32;

fn presses_for_numeric_code(code: &str) -> String {
    assert!(code.ends_with("A"));

    let mut result = String::new();

    // Get the robot arm's current X/Y location above the keypad ("A" key)
    let (mut cur_row, mut cur_col) = NUMERIC_KEYPAD.get(&'A').unwrap();

    for ch in code.chars() {
        // Get the X/Y location of the key in `ch`
        let &(dest_row, dest_col) = NUMERIC_KEYPAD.get(&ch).unwrap();

        // See if we need to move in a different order to avoid
        // the missing key position.
        if cur_row == 3 && dest_col == 0 {
            // Must go up first!
            while dest_row < cur_row {
                result.push('^');
                cur_row -= 1;
            }
        }
        if cur_col == 0 && dest_row == 3 {
            // Must go right first!
            while dest_col > cur_col {
                result.push('>');
                cur_col += 1;
            }
        }

        // Determine which moves, and what order, are needed to move
        // to that position.  Move in order: left, down, right, up.
        while dest_col < cur_col {
            result.push('<');
            cur_col -= 1;
        }
        while dest_row > cur_row {
            result.push('v');
            cur_row += 1;
        }
        while dest_col > cur_col {
            result.push('>');
            cur_col += 1;
        }
        while dest_row < cur_row {
            result.push('^');
            cur_row -= 1;
        }

        // Need to press our "A" button to cause us to press the
        // `ch` button in front of us.
        result.push('A');
    }

    result
}

fn presses_for_directional_code(code: &str) -> String {
    assert!(code.ends_with("A"));

    let mut result = String::new();

    // Get the robot arm's current X/Y location above the keypad ("A" key)
    let (mut cur_row, mut cur_col) = DIRECTIONAL_KEYPAD.get(&'A').unwrap();
    for ch in code.chars() {
        // Get the X/Y location of the key in `ch`
        let &(dest_row, dest_col) = DIRECTIONAL_KEYPAD.get(&ch).unwrap();

        // See if we need to move in a different order to avoid
        // the missing key position.
        if dest_col == 0 {
            // Must go down first!
            while dest_row > cur_row {
                result.push('v');
                cur_row += 1;
                if cur_row == 0 && cur_col == 0 { panic!("Invalid location!"); }
            }
        }
        if cur_col == 0 {
            // Must go right first!
            while dest_col > cur_col {
                result.push('>');
                cur_col += 1;
                if cur_row == 0 && cur_col == 0 { panic!("Invalid location!"); }
            }
        }

        // Determine which moves, and what order, are needed to move
        // to that position.  Move in order: left, down, right, up.
        while dest_col < cur_col {
            result.push('<');
            cur_col -= 1;
            if cur_row == 0 && cur_col == 0 { panic!("Invalid location!"); }
        }
        while dest_row > cur_row {
            result.push('v');
            cur_row += 1;
            if cur_row == 0 && cur_col == 0 { panic!("Invalid location!"); }
        }
        while dest_col > cur_col {
            result.push('>');
            cur_col += 1;
            if cur_row == 0 && cur_col == 0 { panic!("Invalid location!"); }
        }
        while dest_row < cur_row {
            result.push('^');
            cur_row -= 1;
            if cur_row == 0 && cur_col == 0 { panic!("Invalid location!"); }
        }

        // Need to press our "A" button to cause us to press the
        // `ch` button in front of us.
        result.push('A');
    }

    result
}

static NUMERIC_KEYPAD: LazyLock<HashMap<char, (Row, Col)>> = LazyLock::new(||
    HashMap::from([
        ('A', (3,2)),
        ('0', (3,1)),
        ('1', (2,0)),
        ('2', (2,1)),
        ('3', (2,2)),
        ('4', (1,0)),
        ('5', (1,1)),
        ('6', (1,2)),
        ('7', (0,0)),
        ('8', (0,1)),
        ('9', (0,2)),
    ])
);

static DIRECTIONAL_KEYPAD: LazyLock<HashMap<char, (Row, Col)>> = LazyLock::new(||
    HashMap::from([
        ('A', (0,2)),
        ('^', (0,1)),
        ('<', (1,0)),
        ('v', (1,1)),
        ('>', (1,2)),
    ])
);

//
// The code as currently written gives a sub-optimal solution for "379A".
// I think that in some of the cases where the robot needs to move both
// up/down and left/right, that the order I'm choosing makes for longer
// travel for the next robot in the sequence.
//
// In this case, could it be the transition between "3" and "7"?  Should
// I go left, then up?  [Yes!  Robot two's sequence is the same length,
// but the different order allows robot three to generate a shorter
// sequence.]
//
// Is it that "<^" generates a shorter sequence than "^<", or is it
// "A<" followed by "^A" versus "A^" followed by "<A"?  I guess I could
// try "A<^A" and "A^<A".
//
// I think the difference is that with "A<^A", robot two does "v<<" to
// get from "A" to "<", which means robot three can move over "<"
// (expensive) and press "A" twice (inexpensive).  But with "A^<A",
// robot two moves "<" and "v<", which requires an additional left
// and right movement to get back to "<".
//
// I suspect that this means that when we need to move both horizontally
// and vertically, that we should prefer left (furthest from A), then
// down (next furthest from A), then up or right (I don't think the
// order matters).
//
// Using the tests below (with values supplied on Reddit), it seems that
// my general solution fails for code "456A" with 5 robots pressing
// directional keypads.  I haven't investigated to see what is going on,
// and why always choosing directions in the same order isn't working.
//
#[test]
#[allow(non_snake_case)]
fn test_part1_code379A() {
    let input = "\
379A
";
    assert_eq!(part1(input), 64 * 379);
}

#[test]
fn test_part1() {
    let input = "\
029A
980A
179A
456A
379A
";
    assert_eq!(part1(input), 126384);
}

#[test]
fn test_part2_inner() {
    let input = "\
029A
980A
179A
456A
379A
";
    assert_eq!(part2_inner(input, 2), 126384);
}

#[test]
fn test_part2_for_code_brute_456() {
    let code = "456A";
    // assert_eq!(part2_inner(code, 0), 456 * 12);
    assert_eq!(part2_inner(code, 1), 456 * 26);
    assert_eq!(part2_inner(code, 2), 456 * 64);
    assert_eq!(part2_inner(code, 3), 456 * 162);
    assert_eq!(part2_inner(code, 4), 456 * 394);
    assert_eq!(part2_inner(code, 5), 456 * 988);     // This fails.  I'm getting 994.
    assert_eq!(part2_inner(code, 6), 456 * 2434);
    assert_eq!(part2_inner(code, 7), 456 * 6082);
    assert_eq!(part2_inner(code, 8), 456 * 15090);
    assert_eq!(part2_inner(code, 9), 456 * 37576);
    assert_eq!(part2_inner(code, 10), 456 * 93444);
    assert_eq!(part2_inner(code, 11), 456 * 232450);
    assert_eq!(part2_inner(code, 12), 456 * 578314);
    assert_eq!(part2_inner(code, 13), 456 * 1438450);
    assert_eq!(part2_inner(code, 14), 456 * 3578646);
    assert_eq!(part2_inner(code, 15), 456 * 8901822);
    assert_eq!(part2_inner(code, 16), 456 * 22145084);
    assert_eq!(part2_inner(code, 17), 456 * 55087898);
    assert_eq!(part2_inner(code, 18), 456 * 137038728);
    assert_eq!(part2_inner(code, 19), 456 * 340900864);
    assert_eq!(part2_inner(code, 20), 456 * 848032810);
    assert_eq!(part2_inner(code, 21), 456 * 2109590876);
    assert_eq!(part2_inner(code, 22), 456 * 5247866716);
    assert_eq!(part2_inner(code, 23), 456 * 13054736520);
    assert_eq!(part2_inner(code, 24), 456 * 32475283854);
    assert_eq!(part2_inner(code, 25), 456 * 80786362258);
}

#[test]
fn test_part2_others() {
    // My answers for all of these are too high
    assert_eq!(part2_inner("029A", 25), 29 * 82050061710);
    assert_eq!(part2_inner("980A", 25), 980 * 72242026390);
    assert_eq!(part2_inner("179A", 25), 179 * 81251039228);
    assert_eq!(part2_inner("456A", 25), 456 * 80786362258);
    assert_eq!(part2_inner("379A", 25), 379 * 77985628636);
}

#[cfg(test)]
static FULL_INPUT: &str = include_str!("../input.txt");

#[test]
fn test_part1_full() {
    assert_eq!(part1(FULL_INPUT), 94284);
}

#[test]
fn test_part2_full() {
    let result = part2(FULL_INPUT);
    assert_eq!(result, 116_821_732_384_052);
}
