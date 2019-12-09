#![allow(clippy::many_single_char_names)]

use crate::intcode::*;
use aoc_runner_derive::aoc;
use crossbeam_channel::bounded;
use itertools::Itertools;
use rayon::prelude::*;
use std::iter::once;

fn parse_input(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap()
}

#[aoc(day7, part1)]
fn part1_overengineered(input: &str) -> isize {
    let code = parse_input(input);

    let inputs: Vec<(isize, isize, isize, isize, isize)> = (0..=4)
        .permutations(5)
        .map(|v| (v[0], v[1], v[2], v[3], v[4]))
        .collect();

    inputs
        .into_par_iter()
        .map(|ins| {
            let (a, b, c, d, e) = ins;
            let (a_out, b_in) = bounded(1);
            let (b_out, c_in) = bounded(1);
            let (c_out, d_in) = bounded(1);
            let (d_out, e_in) = bounded(1);
            let mut e_out = 0isize;

            let mut a = IntcodeMachine::new(&code, once(a).chain(once(0)), a_out);
            let mut b = IntcodeMachine::new(&code, once(b).chain(b_in), b_out);
            let mut c = IntcodeMachine::new(&code, once(c).chain(c_in), c_out);
            let mut d = IntcodeMachine::new(&code, once(d).chain(d_in), d_out);
            let mut e = IntcodeMachine::new(&code, once(e).chain(e_in), &mut e_out);

            let mut pool = scoped_threadpool::Pool::new(5);

            pool.scoped(|scope| {
                scope.execute(|| a.run());
                scope.execute(|| b.run());
                scope.execute(|| c.run());
                scope.execute(|| d.run());
                scope.execute(|| e.run());
            });

            e_out
        })
        .max()
        .unwrap()
}

#[aoc(day7, part2)]
fn part2_overengineered(input: &str) -> isize {
    let code = parse_input(input);

    let inputs: Vec<(isize, isize, isize, isize, isize)> = (5..=9)
        .permutations(5)
        .map(|v| (v[0], v[1], v[2], v[3], v[4]))
        .collect();

    inputs
        .into_par_iter()
        .map(|ins| {
            let (a, b, c, d, e) = ins;
            let (a_out, b_in) = bounded(1);
            let (b_out, c_in) = bounded(1);
            let (c_out, d_in) = bounded(1);
            let (d_out, e_in) = bounded(1);
            let (e_out, a_in) = bounded(1);

            let mut e_out_end = 0isize;

            let output = &mut (e_out, &mut e_out_end);

            let mut a = IntcodeMachine::new(&code, once(a).chain(once(0).chain(a_in)), a_out);
            let mut b = IntcodeMachine::new(&code, once(b).chain(b_in), b_out);
            let mut c = IntcodeMachine::new(&code, once(c).chain(c_in), c_out);
            let mut d = IntcodeMachine::new(&code, once(d).chain(d_in), d_out);
            let mut e = IntcodeMachine::new(&code, once(e).chain(e_in), output);

            let mut pool = scoped_threadpool::Pool::new(5);

            pool.scoped(|scope| {
                scope.execute(|| a.run());
                scope.execute(|| b.run());
                scope.execute(|| c.run());
                scope.execute(|| d.run());
                scope.execute(|| e.run());
            });

            e_out_end
        })
        .max()
        .unwrap()
}

#[test]
fn part1_examples() {
    let input1 = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
    assert_eq!(part1_overengineered(input1), 43210);

    let input2 = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
    assert_eq!(part1_overengineered(input2), 54321);

    let input3 = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
    assert_eq!(part1_overengineered(input3), 65210);
}

#[test]
fn part2_examples() {
    let input1 =
        "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
    assert_eq!(part2_overengineered(input1), 139_629_729);

    let input2 = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
    assert_eq!(part2_overengineered(input2), 18216);
}
