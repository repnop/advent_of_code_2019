use crate::intcode::*;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap()
}

#[aoc(day2, part1)]
fn part1(bytes: &[isize]) -> isize {
    let mut bytes = bytes.to_vec();

    // before running the program, replace position 1 with the value 12 and
    // replace position 2 with the value 2
    bytes[1] = 12;
    bytes[2] = 2;

    let mut machine = IntcodeMachine::new(&mut bytes, None.into_iter(), ());
    machine.run();

    machine.data()[0]
}

#[aoc(day2, part2)]
fn part2(bytes: &[isize]) -> String {
    let mut mbytes = Vec::with_capacity(bytes.len());
    let desired = 19_690_720;

    let mut iter = None.into_iter();

    for noun in 0..=99 {
        for verb in 0..=99 {
            mbytes.clear();
            mbytes.extend_from_slice(bytes);

            mbytes[1] = noun;
            mbytes[2] = verb;

            let mut machine = IntcodeMachine::new(&mut mbytes, &mut iter, ());
            machine.run();

            if mbytes[0] == desired {
                return format!(
                    "noun: {}, verb: {}, 100 * noun + verb = {}",
                    noun,
                    verb,
                    100 * noun + verb
                );
            }
        }
    }

    unreachable!()
}
