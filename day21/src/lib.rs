use std::{collections::HashMap, sync::LazyLock};

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
// Brute force.  Peak memory usage is about 100GB, during the last pass
// for each line of input.
//
// The web site says my answer is too low.  Am I doing one too few passes?
// I don't think so.  Part 1 has two robots pressing on directional pads,
// and Part 2 has 25.  I think 25 passes is correct.  I modified this code
// to loop 0..2, and it in fact generates the same answer as part 1.
//
fn part2_inner(input: &str, num_robots: u32) -> usize {
    let mut cache = HashMap::new();
    input.lines().map(|line| {
        let numeric_code = line.strip_suffix('A').unwrap().parse::<usize>().unwrap();
        let directional_code = presses_for_numeric_code(line);
        numeric_code * num_presses_for_directional_code(directional_code, num_robots, &mut cache)
    }).sum()
}
pub fn part2(input: &str) -> usize {
    part2_inner(input, 25)
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

#[allow(dead_code)]
fn presses_for_code_brute(code: &str, num_robots: u32) -> String {
    let mut presses = presses_for_numeric_code(code);
    for _ in 0..num_robots {
        presses = presses_for_directional_code(&presses);
    }
    presses
}

fn num_presses_for_directional_code(code: String, num_robots: u32, cache: &mut HashMap<(String, u32), usize>) -> usize {
    if let Some(n) = cache.get(&(code.clone(), num_robots)) {
        return *n;
    }
    if num_robots == 0 {
        let len = code.len();
        cache.insert((code, 0), len);
        return len;
    }

    let presses = presses_for_directional_code(&code);

    // Break `code` into a sequence of strings, each ending with "A".
    let result = presses.split_inclusive('A')
        .map(|s| num_presses_for_directional_code(s.to_owned(), num_robots-1, cache))
        .sum();
    
    cache.insert((code, num_robots), result);

    result
}

#[allow(dead_code)]
fn num_presses_for_code(code: &str, num_robots: u32) -> usize {
    let mut cache = HashMap::new();
    let directional_code = presses_for_numeric_code(code);
    num_presses_for_directional_code(directional_code, num_robots, &mut cache)
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
fn test_presses_for_code_brute_456() {
    let code = "456A";
    assert_eq!(presses_for_code_brute(code, 0).len(), 12);
    assert_eq!(presses_for_code_brute(code, 1).len(), 26);
    assert_eq!(presses_for_code_brute(code, 2).len(), 64);
    assert_eq!(presses_for_code_brute(code, 3).len(), 162);
    assert_eq!(presses_for_code_brute(code, 4).len(), 394);
    assert_eq!(presses_for_code_brute(code, 5).len(), 988);     // This fails.  I'm getting 994.
    assert_eq!(presses_for_code_brute(code, 6).len(), 2434);
    assert_eq!(presses_for_code_brute(code, 7).len(), 6082);
    assert_eq!(presses_for_code_brute(code, 8).len(), 15090);
    assert_eq!(presses_for_code_brute(code, 9).len(), 37576);
    assert_eq!(presses_for_code_brute(code, 10).len(), 93444);
    assert_eq!(presses_for_code_brute(code, 11).len(), 232450);
    assert_eq!(presses_for_code_brute(code, 12).len(), 578314);
    assert_eq!(presses_for_code_brute(code, 13).len(), 1438450);
    assert_eq!(presses_for_code_brute(code, 14).len(), 3578646);
    assert_eq!(presses_for_code_brute(code, 15).len(), 8901822);
    assert_eq!(presses_for_code_brute(code, 16).len(), 22145084);
    assert_eq!(presses_for_code_brute(code, 17).len(), 55087898);
    assert_eq!(presses_for_code_brute(code, 18).len(), 137038728);
    assert_eq!(presses_for_code_brute(code, 19).len(), 340900864);
    assert_eq!(presses_for_code_brute(code, 20).len(), 848032810);
    assert_eq!(presses_for_code_brute(code, 21).len(), 2109590876);
    assert_eq!(presses_for_code_brute(code, 22).len(), 5247866716);
    assert_eq!(presses_for_code_brute(code, 23).len(), 13054736520);
    assert_eq!(presses_for_code_brute(code, 24).len(), 32475283854);
    assert_eq!(presses_for_code_brute(code, 25).len(), 80786362258);
}

#[test]
fn test_num_presses_for_code_456() {
    let code = "456A";
    assert_eq!(num_presses_for_code(code, 0), 12);
    assert_eq!(num_presses_for_code(code, 1), 26);
    assert_eq!(num_presses_for_code(code, 2), 64);
    assert_eq!(num_presses_for_code(code, 3), 162);
    assert_eq!(num_presses_for_code(code, 4), 394);
    assert_eq!(num_presses_for_code(code, 5), 988);     // This fails.  I'm getting 994.
    assert_eq!(num_presses_for_code(code, 6), 2434);
    assert_eq!(num_presses_for_code(code, 7), 6082);
    assert_eq!(num_presses_for_code(code, 8), 15090);
    assert_eq!(num_presses_for_code(code, 9), 37576);
    assert_eq!(num_presses_for_code(code, 10), 93444);
    assert_eq!(num_presses_for_code(code, 11), 232450);
    assert_eq!(num_presses_for_code(code, 12), 578314);
    assert_eq!(num_presses_for_code(code, 13), 1438450);
    assert_eq!(num_presses_for_code(code, 14), 3578646);
    assert_eq!(num_presses_for_code(code, 15), 8901822);
    assert_eq!(num_presses_for_code(code, 16), 22145084);
    assert_eq!(num_presses_for_code(code, 17), 55087898);
    assert_eq!(num_presses_for_code(code, 18), 137038728);
    assert_eq!(num_presses_for_code(code, 19), 340900864);
    assert_eq!(num_presses_for_code(code, 20), 848032810);
    assert_eq!(num_presses_for_code(code, 21), 2109590876);
    assert_eq!(num_presses_for_code(code, 22), 5247866716);
    assert_eq!(num_presses_for_code(code, 23), 13054736520);
    assert_eq!(num_presses_for_code(code, 24), 32475283854);
    assert_eq!(num_presses_for_code(code, 25), 80786362258);
}

#[test]
fn test_part2_others() {
    // My answers for all of these are too high
    assert_eq!(num_presses_for_code("029A", 25), 82050061710);
    assert_eq!(num_presses_for_code("980A", 25), 72242026390);
    assert_eq!(num_presses_for_code("179A", 25), 81251039228);
    assert_eq!(num_presses_for_code("456A", 25), 80786362258);
    assert_eq!(num_presses_for_code("379A", 25), 77985628636);
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
    assert!(result >  96_631_806_002_350);
    assert!(result < 132_929_214_388_818);
}
