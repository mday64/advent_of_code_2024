use std::collections::HashMap;
use nom::{branch::alt, bytes::complete::tag, character::complete::{alphanumeric1, newline}, multi::many1, sequence::{delimited, separated_pair, terminated}, IResult, Parser};

#[allow(clippy::upper_case_acronyms)]
enum Operation {
    AND, OR, XOR
}
impl Operation {
    fn evaluate(&self, src1: bool, src2: bool) -> bool {
        match self {
            Operation::AND => src1 && src2,
            Operation::OR => src1 || src2,
            Operation::XOR => src1 ^ src2,
        }
    }
}

struct Gate<'a> {
    operation: Operation,
    src1: &'a str,
    src2: &'a str,
    output: &'a str
}

pub fn part1(input: &str) -> u64 {
    // Parse the input
    let (_, (mut wires, mut gates)) = parse_input(input).expect("valid input");

    // Simulate all of the gates
    while !gates.is_empty() {
        gates.retain(|gate| {
            if let Some(src1) = wires.get(gate.src1) {
                if let Some(src2) = wires.get(gate.src2) {
                    let output = gate.operation.evaluate(*src1, *src2);
                    wires.insert(gate.output, output);
                    return false;
                }
            }
            true
        });
    }
    
    // Construct the output value
    let mut result = 0;
    for bit in (0..64).rev() {
        let wire_name = format!("z{:02}", bit);
        let value = wires.get(&wire_name[..]).unwrap_or(&false);
        result = result * 2 + if *value { 1 } else { 0 };
    }
    result
}

pub fn part2(_input: &str) -> String {
    "World".to_string()
}

fn parse_wire(input: &str) -> IResult<&str, (&str, bool)> {
    let (input, (name, value)) = separated_pair(alphanumeric1, tag(": "), alt((tag("0"), tag("1")))).parse(input)?;
    let value = value == "1";
    Ok((input, (name, value)))
}

fn parse_op(input: &str) -> IResult<&str, Operation> {
    let (input, op_str) = delimited(tag(" "), alt((tag("AND"), tag("OR"), tag("XOR"))), tag(" ")).parse(input)?;
    match op_str {
        "AND" => Ok((input, Operation::AND)),
        "OR" => Ok((input, Operation::OR)),
        "XOR" => Ok((input, Operation::XOR)),
        _ => panic!("Invalid operation")
    }
}

fn parse_gate(input: &str) -> IResult<&str, Gate> {
    let (input, (src1, operation, src2, _, output)) = (alphanumeric1, parse_op, alphanumeric1, tag(" -> "), alphanumeric1).parse(input)?;
    Ok((input, Gate{operation, src1, src2, output}))
}

fn parse_input(input: &str) -> IResult<&str, (HashMap<&str, bool>, Vec<Gate>)> {
    let (input, (wires, gates)) = separated_pair(
        many1(terminated(parse_wire, newline)),
        newline,
        many1(terminated(parse_gate, newline)),
    ).parse(input)?;
    Ok((input, (wires.into_iter().collect(), gates)))
}

#[test]
fn test_part1_small() {
    let input = "\
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
";
    assert_eq!(part1(input), 4);
}

#[test]
fn test_part1_larger() {
    let input = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";
    assert_eq!(part1(input), 2024);
}

#[test]
fn test_part2() {
    let input = "Hello, World!";
    assert_eq!(part2(input), "World");
}
