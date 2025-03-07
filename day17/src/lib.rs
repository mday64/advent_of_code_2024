use itertools::Itertools;
use nom::{bytes::tag, character::complete::{self, newline}, combinator::all_consuming, multi::separated_list1, sequence::{delimited, pair}, IResult, Parser};

struct Computer {
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,
    pc: usize,
    program: Vec<u8>,
    output: Vec<u32>
}

impl Computer {
    fn new(reg_a: u32, reg_b: u32, reg_c: u32, program: Vec<u8>) -> Computer {
        Computer { reg_a, reg_b, reg_c, pc: 0, program, output: vec![] }
    }

    fn evaluate_combo_arg(&self, argument: u8) -> u32 {
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
                self.reg_b ^= argument as u32;
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
                self.output.push(self.evaluate_combo_arg(argument) & 0x7);
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
        self.output.iter()
            .map(|i| i.to_string())
            .join(",")
    }
}

fn parse_input(input: &str) -> IResult<&str, Computer> {
    let (remaining, (reg_a, reg_b, reg_c, program)) = all_consuming((
        delimited(tag("Register A: "), complete::u32, newline),
        delimited(tag("Register B: "), complete::u32, newline),
        delimited(tag("Register C: "), complete::u32, pair(newline, newline)),
        delimited(tag("Program: "), separated_list1(tag(","), complete::u8), newline)
    )).parse(input)?;
    Ok((remaining, Computer::new(reg_a, reg_b, reg_c, program)))
}

pub fn part1(input: &str) -> String {
    let mut computer = parse_input(input).expect("invalid input").1;
    computer.run_program();
    computer.get_output()
}

pub fn part2(_input: &str) -> String {
    "World".to_string()
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
    let input = "Hello, World!";
    assert_eq!(part2(input), "World");
}
