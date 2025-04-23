use std::{collections::HashMap, mem::swap};
use nom::{branch::alt, bytes::complete::tag, character::complete::{alphanumeric1, newline}, multi::many1, sequence::{delimited, separated_pair, terminated}, IResult, Parser};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
struct Gate<'a> {
    operation: Operation,
    inputs: [&'a str; 2],
    output: &'a str
}

impl Gate<'_> {
    #[allow(dead_code)]
    fn print(&self) {
        let op_str = match self.operation {
            Operation::AND => "AND",
            Operation::OR => "OR",
            Operation::XOR => "XOR",
        };
        println!("{} {} {} -> {}", self.inputs[0], op_str, self.inputs[1], self.output);
    }
}

pub fn part1(input: &str) -> u64 {
    // Parse the input
    let (_, (mut wires, mut gates)) = parse_input(input).expect("valid input");

    // Simulate all of the gates
    while !gates.is_empty() {
        gates.retain(|gate| {
            if let Some(in0) = wires.get(gate.inputs[0]) {
                if let Some(in1) = wires.get(gate.inputs[1]) {
                    let output = gate.operation.evaluate(*in0, *in1);
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

//
// The outputs from 4 pairs of gates need to be swapped in order to
// turn the gates into a full adder.  (Our input adds 44-bit integers,
// and produces a 45-bit output.)
//
// We could try to solve it by inspection of the input (i.e. by hand).
//
// We could try to brute force it by trying all combinations of 4 pairs
// of gates.  There are 222 gates.  That would take far too long.
//
// We can try examining which outputs are wrong, see which gates are
// involved in computing their value, and brute force try to find the
// pairs.  This might make the problem tractable.
//
// It might be more efficient to work from least significant bits to
// more significant bits (because of carries).  We can hope that we
// need to swap at most one pair of gates for each new bit.
//
// I wonder if the input wire values we're given are enough to find
// all of the errors, or if we need to exercise more input combinations.
//
// As I read the description, we don't need to change the names of any
// gate inputs -- just outputs.  To add the low N bits, we need the
// x{0..N} and y{0..N} inputs, will produce the z{0..=N} outputs, and
// will produce a carry out.  Then the next bit will take that carry out
// as a carry in, and produce a new carry out.  If the carry out from
// bit N doesn't match the carry in to bit N+1, then we know that
// the carry out name is wrong (and what it should be -- so we can
// find the matching gate in the pair).
//
// If we assume that all of the outputs ("z..") will eventually get a
// value, then the wire swaps are constrained, and probably limited to
// swapping the outputs for two gates for a given bit number.
//
// For a single bit (not bit 0), there will be 5 gates:
// (carry in: ccc)
// xnn XOR ynn -> ddd   <- find this by inputs
// ccc XOR ddd -> znn   <- find this by output?
// xnn AND ynn -> eee   <- find this by inputs
// ccc AND ddd -> fff   <- same inputs as znn output
// eee OR fff -> ggg
// (where ggg becomes the carry in to the next bit)
//
// When searching for a gate with xnn and ynn inputs, it is sufficient
// to just look for one input being xnn.
//
// TODO: I think we can solve this in a purely automated way, that doesn't
// depend on inspection of the input, and me trying to deduce what swap
// would fix the problem.
//
// The idea would be to test the correct operation of each bit, one at
// a time.  That is supply the 8 possible combinations of inputs (x, y,
// carry in) for a single bit, and verify that the output bit is set
// correctly.  If not, find the gates with the given bit number as
// input (x or y) or output (z).  Then gather all of the wires associated
// with those gates, and try swapping each unique pair, until the
// output is correct.
//
pub fn part2(input: &str) -> String {
    // From manual inspection, bits 0..=8 are fine, and we have one swap
    // involving bit 9: "hnd" and "z09".
    // Note the z09 output was an AND, not an XOR
    let mut crossed_wires: Vec<String> = vec![];

    let (_input, (_wires, mut gates)) = parse_input(input).expect("valid input");
    let gate = find_gate("x00", Operation::XOR, &gates);
    if gate.output != "z00" {
        println!("Swap: z00, {}", gate.output);
        return "TODO".to_string();
    }
    let mut carry = find_gate("x00", Operation::AND, &gates).output;

    for bit in 1..=44 {
        // eprintln!("Bit {bit}...");
        let x_str = format!("x{bit:02}");
        let z_str = format!("z{bit:02}");
        
        let mut znn = *gates.iter().find(|g| g.output==z_str).unwrap();
        if znn.operation != Operation::XOR {
            let mut other = find_gate(carry, Operation::XOR, &gates);
            let other_out = other.output;
            // eprintln!("{z_str} <-> {}", other_out);
            swap_outputs(&z_str, other_out, &mut gates);
            crossed_wires.push(z_str);
            crossed_wires.push(other_out.to_string());
            other.output = znn.output;
            znn = other;
        }
        assert!(znn.inputs.contains(&carry));
        let mut ddd = find_gate(&x_str, Operation::XOR, &gates);
        let mut eee = find_gate(&x_str, Operation::AND, &gates);
        if !znn.inputs.contains(&ddd.output) && znn.inputs.contains(&eee.output) {
            // eprintln!("{} <-> {}", ddd.output, eee.output);
            crossed_wires.push(ddd.output.to_string());
            crossed_wires.push(eee.output.to_string());
            swap_outputs(ddd.output, eee.output, &mut gates);
            swap(&mut ddd.output, &mut eee.output);
        }
        assert!(znn.inputs.contains(&ddd.output));
        let fff = find_gate(carry, Operation::AND, &gates);
        assert!(fff.inputs.contains(&ddd.output));
        let ggg = find_gate(fff.output, Operation::OR, &gates);
        assert!(ggg.inputs.contains(&eee.output));

        carry = ggg.output;
    }
    assert_eq!(carry, "z45");
    assert_eq!(crossed_wires.len(), 8);

    crossed_wires.sort();
    crossed_wires.join(",")
}

fn find_gate<'a>(src: &str, operation: Operation, gates: &[Gate<'a>]) -> Gate<'a> {
    for gate in gates {
        if gate.operation == operation && (gate.inputs.contains(&src)) {
            return *gate;
        }
    }
    panic!("gate not found!");
}

fn swap_outputs(wire1: &str, wire2: &str, gates: &mut [Gate]) {
    let mut gate1 = None;
    let mut gate2 = None;
    for gate in gates.iter_mut() {
        if gate.output == wire1 {
            gate1 = Some(gate);
        } else if gate.output == wire2 {
            gate2 = Some(gate);
        }
    }
    let gate1 = gate1.unwrap();
    let gate2 = gate2.unwrap();
    swap(&mut gate1.output, &mut gate2.output);
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
    let inputs = [src1, src2];
    Ok((input, Gate{operation, inputs, output}))
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

#[cfg(test)]
static FULL_INPUT: &str = include_str!("../input.txt");

#[test]
fn test_part1_full() {
    assert_eq!(part1(FULL_INPUT), 53190357879014);
}

#[test]
fn test_part2_full() {
    assert_eq!(part2(FULL_INPUT), "bks,hnd,nrn,tdv,tjp,z09,z16,z23");
}
