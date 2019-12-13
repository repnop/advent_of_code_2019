#![allow(clippy::trivially_copy_pass_by_ref)]

#[allow(unused_imports)]
#[macro_use]
extern crate itertools;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;

pub mod intcode;

aoc_runner_derive::aoc_lib! { year = 2019 }
