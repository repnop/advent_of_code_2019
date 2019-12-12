#![allow(clippy::zero_prefixed_literal)]

use advent_of_code_2019::intcode::*;
use std::io::{stdin, stdout, BufRead};

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();
    let mut stdout = stdout();

    let mut program: Vec<isize> = vec![
        01005, 0, 512, // jit pos(0), imm(512)
    ];

    program.resize(4096, 0);

    #[rustfmt::skip]
    let input_program = [
        01101, 0, 0, 0,             // add imm(0), imm(0), pos(0),
        01101, 0, 0, 2,             // add imm(0), imm(0), pos(2),
        00003, 1000,                // input pos(1000)
        20101, 0, 1000, 0,          // add imm(0), pos(1000), rel(0)
        00109, 1,                   // arel imm(1)
        01007, 1000, 99999, 1001,   // lt pos(90), imm(99999), pos(1001)
        01006, 1001, 0,             // jif pos(1001), imm(0)
        01005, 1001, 520,           // jit pos(1001), imm(520)
    ];

    program[512..(512 + input_program.len())].copy_from_slice(&input_program);

    let mut machine = IntcodeMachine::new(
        &program,
        stdin.lines().filter_map(|l| l.ok()?.parse::<isize>().ok()),
        &mut stdout,
    );

    machine.run();
}
