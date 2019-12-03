use aoc_runner_derive::{aoc, aoc_generator};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

type TwoWires = (Vec<Path>, Vec<Path>);

#[aoc_generator(day3)]
fn gen(input: &str) -> TwoWires {
    let mut lines = input.lines();

    (
        lines
            .next()
            .unwrap()
            .split(',')
            .map(FromStr::from_str)
            .collect::<Result<_, _>>()
            .unwrap(),
        lines
            .next()
            .unwrap()
            .split(',')
            .map(FromStr::from_str)
            .collect::<Result<_, _>>()
            .unwrap(),
    )
}

#[derive(Clone, Copy)]
struct Path {
    dir: Direction,
    len: i32,
}

impl FromStr for Path {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, len) = s.split_at(1);
        let len = len.parse().unwrap();

        let dir = match dir {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => return Err(()),
        };

        Ok(Self { dir, len })
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn into_coord_offset(self) -> (i32, i32) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
        }
    }
}

#[aoc(day3, part1)]
fn part1(input: &TwoWires) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut wire_1 = HashSet::with_capacity(512);
    for path in &input.0 {
        let (offset_x, offset_y) = path.dir.into_coord_offset();

        for _ in 0..path.len {
            x += offset_x;
            y += offset_y;

            wire_1.insert((x, y));
        }
    }

    let mut x = 0;
    let mut y = 0;
    let mut wire_2 = HashSet::with_capacity(512);
    for path in &input.1 {
        let (offset_x, offset_y) = path.dir.into_coord_offset();

        for _ in 0..path.len {
            x += offset_x;
            y += offset_y;

            wire_2.insert((x, y));
        }
    }

    let intersections = &wire_1 & &wire_2;

    intersections
        .into_iter()
        .map(|coord| manhatten((0, 0), coord))
        .min()
        .unwrap()
}

fn manhatten((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

#[aoc(day3, part2)]
fn part2(input: &TwoWires) -> i32 {
    let mut first_wire_path = HashMap::with_capacity(512);

    let (mut x, mut y) = (0, 0);
    let mut total_steps = 0;
    for path in &input.0 {
        let (offset_x, offset_y) = path.dir.into_coord_offset();

        for _ in 0..path.len {
            x += offset_x;
            y += offset_y;
            total_steps += 1;

            first_wire_path.entry((x, y)).or_insert(total_steps);
        }
    }

    x = 0;
    y = 0;
    total_steps = 0;

    let mut intersections = Vec::with_capacity(512);

    for path in &input.1 {
        let (offset_x, offset_y) = path.dir.into_coord_offset();

        for _ in 0..path.len {
            x += offset_x;
            y += offset_y;
            total_steps += 1;

            if let Some(total) = first_wire_path.get(&(x, y)) {
                intersections.push(total + total_steps);
            }
        }
    }

    intersections.into_iter().min().unwrap()
}

#[test]
fn example_1() {
    let inp = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    assert_eq!(part2(&gen(inp)), 610);
}
