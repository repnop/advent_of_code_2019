use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1(vals: &[i32]) -> i32 {
    vals.iter().copied().fold(0, |mut acc, mass| {
        acc += mass / 3 - 2;
        acc
    })
}

#[aoc(day1, part2)]
fn part2(vals: &[i32]) -> i32 {
    vals.iter().copied().fold(0, |mut acc, mass| {
        let mut fuel_mass = mass / 3 - 2;
        while fuel_mass > 0 {
            acc += fuel_mass;
            fuel_mass = fuel_mass / 3 - 2;
        }
        acc
    })
}
