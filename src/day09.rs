use crate::intcode::*;
use aoc_runner_derive::aoc;
use std::iter::once;

fn parse_input(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap()
}

#[aoc(day9, part1)]
fn part1(input: &str) -> isize {
    let input = parse_input(input);
    let mut output = 0isize;
    let mut machine = IntcodeMachine::new(&input, once(1), &mut output);

    machine.run();

    output
}

#[aoc(day9, part2)]
fn part2(input: &str) -> isize {
    let input = parse_input(input);
    let mut output = 0isize;
    let mut machine = IntcodeMachine::new(&input, once(2), &mut output);

    machine.run();

    output
}
