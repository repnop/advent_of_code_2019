use crate::intcode::*;
use aoc_runner_derive::{aoc, aoc_generator};
use std::iter::once;

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap()
}

#[aoc(day5, part1)]
fn part1(input: &[isize]) -> isize {
    let bytes = input.to_vec();
    let mut output = 0isize;

    let mut machine = IntcodeMachine::new(&bytes, once(1), &mut output);
    machine.run();

    output
}

#[aoc(day5, part2)]
fn part2(input: &[isize]) -> isize {
    let bytes = input.to_vec();
    let mut output = 0isize;

    let mut machine = IntcodeMachine::new(&bytes, once(5), &mut output);
    machine.run();

    output
}
