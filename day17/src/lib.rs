use itertools::Itertools;
use nom::{bytes::tag, character::complete::{self, newline}, combinator::all_consuming, multi::separated_list1, sequence::{delimited, pair}, IResult, Parser};
use num::pow;

struct Computer {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    pc: usize,
    program: Vec<u8>,
    output: Vec<u8>
}

impl Computer {
    fn new(reg_a: u64, reg_b: u64, reg_c: u64, program: Vec<u8>) -> Computer {
        Computer { reg_a, reg_b, reg_c, pc: 0, program, output: vec![] }
    }

    fn reset(&mut self, reg_a: u64, reg_b: u64, reg_c: u64) {
        self.reg_a = reg_a;
        self.reg_b = reg_b;
        self.reg_c = reg_c;
        self.pc = 0;
        self.output.clear();
    }

    fn evaluate_combo_arg(&self, argument: u8) -> u64 {
        match argument {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("invalid argument")
        }
    }

    fn execute_instruction(&mut self, opcode: u8, argument: u8) {
        match opcode {
            0 => {          // Divide/shift [combo]
                self.reg_a >>= self.evaluate_combo_arg(argument);
            }
            1 => {          // xor [literal]
                self.reg_b ^= argument as u64;
            }
            2 => {          // mod 8 [combo]
                self.reg_b = self.evaluate_combo_arg(argument) & 0x7;
            }
            3 => {          // jump if not zero [literal]
                if self.reg_a != 0 {
                    self.pc = argument as usize;
                }
            }
            4 => {          // xor
                self.reg_b ^= self.reg_c;
            }
            5 => {          // out
                self.output.push(self.evaluate_combo_arg(argument) as u8 & 0x7);
            }
            6 => {          // Divide/shift [combo]
                self.reg_b = self.reg_a >> self.evaluate_combo_arg(argument);
            }
            7 => {          // Divide/shift [combo]
                self.reg_c = self.reg_a >> self.evaluate_combo_arg(argument);
            }
            _ => panic!("invalid opcode")
        }
    }

    fn run_program(&mut self) {
        while self.pc < self.program.len() {
            let opcode = self.program[self.pc];
            let argument = self.program[self.pc + 1];
            self.pc += 2;
            self.execute_instruction(opcode, argument);
        }
    }

    fn get_output(&self) -> String {
        self.output.iter().join(",")
    }
}

fn parse_input(input: &str) -> IResult<&str, Computer> {
    let (remaining, (reg_a, reg_b, reg_c, program)) = all_consuming((
        delimited(tag("Register A: "), complete::u64, newline),
        delimited(tag("Register B: "), complete::u64, newline),
        delimited(tag("Register C: "), complete::u64, pair(newline, newline)),
        delimited(tag("Program: "), separated_list1(tag(","), complete::u8), newline)
    )).parse_complete(input)?;
    Ok((remaining, Computer::new(reg_a, reg_b, reg_c, program)))
}

pub fn part1(input: &str) -> String {
    let mut computer = parse_input(input).expect("invalid input").1;
    computer.run_program();
    computer.get_output()
}

//
// I'll bet that the code is essentially outputting the value of the A
// register in base 8, least significant digit first, possibly with
// some XOR-ing of those digits.
//
// The mod-8 most-significant-digit seems to map:
// 1 => 4
// 2 => 5
// 3 => 7
// 4 => 1
// 5 => 0
// 6 => 3
// 7 => 2
//
// I think I can brute force this by finding one digit at a time, starting with
// the last (most significant) digit.  Try base-8 digits from 0 to 7, until
// the corresponding digit of the program matches, then the next most
// significant digit, and so on.
//
pub fn part2(input: &str) -> u64 {
    let mut computer = parse_input(input).expect("invalid input").1;
    let mut place_value = pow(8, computer.program.len() - 1);
    let mut digits_to_find = computer.program.len();
    let mut a = place_value;
    computer.reset(a, 0, 0);
    computer.run_program();
    assert_eq!(computer.output.len(), computer.program.len());

    while digits_to_find > 0 {
        while computer.output[digits_to_find - 1] != computer.program[digits_to_find - 1] {
            a += place_value;
            computer.reset(a, 0, 0);
            computer.run_program();
        }
        digits_to_find -= 1;
        place_value /= 8;
    }

    //
    // There's something else weird going on with the encoding that sometimes
    // causes more significant digits to change.  So time for more brute force.
    //
    while computer.output != computer.program {
        a += 1;
        computer.reset(a, 0, 0);
        computer.run_program();
    }

    a
}

#[test]
fn test_part1() {
    let input = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";
    assert_eq!(part1(input), "4,6,3,5,6,3,5,2,1,0");
}

#[test]
fn test_part2() {
    let input = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";
    assert_eq!(part2(input), 117440);
}

#[cfg(test)]
static FULL_INPUT: &str = include_str!("../input.txt");

#[test]
fn test_part1_full() {
    assert_eq!(part1(FULL_INPUT), "2,3,4,7,5,7,3,0,7");
}

#[test]
fn test_part2_full() {
    assert_eq!(part2(FULL_INPUT), 190384609508367);
}
