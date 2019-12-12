use aoc_runner_derive::{aoc, aoc_generator};
use crossbeam_channel::{bounded, Receiver, Sender};

#[aoc_generator(day11)]
fn gen(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap()
}

#[derive(Debug, Clone, Copy)]
struct Point2 {
    x: isize,
    y: isize,
}

enum Message {
    Command(isize),
    Halted,
}

struct Robot {
    position: Point2,
    channel_in: Receiver<Message>,
}

#[aoc(day11, part1)]
fn part1(input: &[isize]) -> usize {
    0
}
