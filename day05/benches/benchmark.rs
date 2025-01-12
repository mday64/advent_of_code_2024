use day05::{ part1, part2, both_parts, both_parts_faster, both_parts_hashset, both_parts_nom };

fn main() {
    divan::main();
}

const INPUT: &str = include_str!("../input.txt");

#[divan::bench]
fn bench_part1() {
    part1(INPUT);
}

#[divan::bench]
fn bench_part2() {
    part2(INPUT);
}

#[divan::bench]
fn bench_both_parts() {
    both_parts(INPUT);
}

#[divan::bench]
fn bench_both_parts_faster() {
    both_parts_faster(INPUT);
}

#[divan::bench]
fn bench_both_parts_hashset() {
    both_parts_hashset(INPUT);
}

#[divan::bench]
fn bench_both_parts_nom() {
    both_parts_nom(INPUT);
}
