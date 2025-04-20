use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> usize {
    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.lines() {
        let (left, right) = line.split_once('-').expect("hyphen");
        // Insert connections such that the strings point to "larger" strings
        if left < right {
            connections.entry(left).or_default().insert(right);
        } else {
            connections.entry(right).or_default().insert(left);
        }
    }

    let mut result = 0;
    for (first, seconds) in &connections {
        for second in seconds {
            if let Some(thirds) = connections.get(second) {
                for third in thirds {
                    if (first.starts_with("t") || second.starts_with("t") || third.starts_with("t"))
                        && seconds.contains(third)
                    {
                        // println!("{first},{second},{third}");
                        result += 1;
                    }
                }
            }
        }
    }
    result
}

pub fn part2(_input: &str) -> String {
    "World".to_string()
}

#[test]
fn test_part1() {
    let input = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";
    assert_eq!(part1(input), 7);
}

#[test]
fn test_part2() {
    let input = "Hello, World!";
    assert_eq!(part2(input), "World");
}
