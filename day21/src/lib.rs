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
pub fn part1(input: &str) -> usize {
    input.lines().map(|line| {
        let code = line.strip_suffix('A').unwrap().parse::<usize>().unwrap();
        let robot1 = presses_for_code(line, &NUMERIC_KEYPAD);
        let robot2 = presses_for_code(&robot1, &DIRECTIONAL_KEYPAD);
        let robot3 = presses_for_code(&robot2, &DIRECTIONAL_KEYPAD);
        println!("{line}\n{robot1}\n{robot2}\n{robot3}\n");
        code * robot3.len()
    }).sum()
}

pub fn part2(_input: &str) -> String {
    "World".to_string()
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
fn presses_for_code(code: &str, keypad: &HashMap<char, (Row, Col)>) -> String {
    assert!(code.ends_with("A"));

    let mut result = String::new();

    // Get the robot arm's current X/Y location above the keypad ("A" key)
    let (mut cur_row, mut cur_col) = keypad.get(&'A').unwrap();

    for ch in code.chars() {
        // Get the X/Y location of the key in `ch`
        let &(dest_row, dest_col) = keypad.get(&ch).unwrap();

        // Determine which moves, and what order, are needed to move
        // to that position.  Move in order: right, up, down, left.
        while dest_col > cur_col {
            result.push('>');
            cur_col += 1;
        }
        while dest_row < cur_row {
            result.push('^');
            cur_row -= 1;
        }
        while dest_row > cur_row {
            result.push('v');
            cur_row += 1;
        }
        while dest_col < cur_col {
            result.push('<');
            cur_col -= 1;
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

#[test]
#[allow(non_snake_case)]
fn code029A_presses() {
    assert_eq!(presses_for_code("029A", &NUMERIC_KEYPAD), "<A^A>^^AvvvA");
}

#[test]
#[allow(non_snake_case)]
fn code029A_presses_two() {
    let robot_one = presses_for_code("029A", &NUMERIC_KEYPAD);
    assert_eq!(robot_one, "<A^A>^^AvvvA");
    let robot_two = presses_for_code(&robot_one, &DIRECTIONAL_KEYPAD);
    assert_eq!(robot_two, "v<<A>>^A<A>AvA^<AA>Av<AAA>^A");
}

#[test]
#[allow(non_snake_case)]
fn code029A_presses_three() {
    let robot_one = presses_for_code("029A", &NUMERIC_KEYPAD);
    assert_eq!(robot_one, "<A^A>^^AvvvA");
    let robot_two = presses_for_code(&robot_one, &DIRECTIONAL_KEYPAD);
    assert_eq!(robot_two, "v<<A>>^A<A>AvA^<AA>Av<AAA>^A");
    let robot_three = presses_for_code(&robot_two, &DIRECTIONAL_KEYPAD);
    assert_eq!(robot_three, "v<A<AA>>^AvAA^<A>Av<<A>>^AvA^Av<A>^A<Av<A>>^AAvA^Av<A<A>>^AAAvA^<A>A");
}

//
// The code as currently written gives a sub-optimal solution for "379A".
// I think that in some of the cases where the robot needs to move both
// up/down and left/right, that the order I'm choosing makes for longer
// travel for the next robot in the sequence.
//
// In this case, could it be the transition between "3" and "7"?  Should
// I go left, then up?
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
fn test_part2() {
    let input = "Hello, World!";
    assert_eq!(part2(input), "World");
}
