use day03::{ part1, part1_many_till, part2, part2_state_machine };

fn main() {
    divan::main();
}

const INPUT: &str = include_str!("../input.txt");

#[divan::bench]
fn bench_part1() {
    part1(INPUT);
}

#[divan::bench]
fn bench_part1_many_till() {
    part1_many_till(INPUT);
}

#[divan::bench]
fn bench_part2() {
    part2(INPUT);
}

#[divan::bench]
fn bench_part2_state_machine() {
    part2_state_machine(INPUT);
}
