use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;

#[derive(PartialEq, Clone, Copy, Debug)]
struct Answer((usize, usize), usize);

impl std::fmt::Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}) = {}", (self.0).0, (self.0).1, self.1)
    }
}

impl From<((usize, usize), usize)> for Answer {
    fn from(n: ((usize, usize), usize)) -> Self {
        Self(n.0, n.1)
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Asteroid,
    Space,
}

impl Tile {
    fn is_space(self) -> bool {
        match self {
            Tile::Space => true,
            _ => false,
        }
    }

    fn is_asteroid(self) -> bool {
        !self.is_space()
    }
}

struct Field {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Field {
    fn new(tiles: Vec<Tile>, width: usize, height: usize) -> Self {
        Self { tiles, width, height }
    }

    fn at(&self, x: usize, y: usize) -> Tile {
        self.tiles[y * self.height + x]
    }

    fn asteroid_points<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        let points = (0..self.width).map(move |x| (0..self.height).map(move |y| (x, y)));

        points.flatten().filter(move |(x, y)| self.at(*x, *y).is_asteroid())
    }
}

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for tiles in self.tiles.chunks_exact(self.width) {
            for tile in tiles {
                write!(f, "{}", if tile.is_asteroid() { '#' } else { '.' })?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[aoc_generator(day10)]
fn gen(input: &str) -> Field {
    let width = input.lines().nth(0).unwrap().chars().count();
    let height = input.lines().count();
    let mut tiles = Vec::with_capacity(width * height);

    tiles.extend(input.lines().flat_map(|line| {
        line.trim().chars().map(|c| match c {
            '#' => Tile::Asteroid,
            '.' => Tile::Space,
            _ => unreachable!(),
        })
    }));

    Field::new(tiles, width, height)
}

#[aoc(day10, part1)]
fn part1(field: &Field) -> Answer {
    let coords: Vec<_> =
        (0..field.width).map(move |x| (0..field.height).map(move |y| (x, y))).flatten().collect();

    coords
        .into_par_iter()
        .filter(|&(x, y)| field.at(x, y).is_asteroid())
        .map(|(x, y)| ((x, y), asteroid_count(field, x, y)))
        .max_by_key(|res| res.1)
        .unwrap()
        .into()
}

fn asteroid_count(field: &Field, x: usize, y: usize) -> usize {
    let mut count = 0;

    for coord in field.asteroid_points() {
        let mut blocked = false;

        if coord == (x, y) {
            continue;
        }

        if coord.0 == x {
            let start = coord.1.min(y);
            let end = coord.1.max(y);

            for y in start + 1..end {
                if field.at(x, y).is_asteroid() {
                    blocked = true;
                    break;
                }
            }
        } else if coord.1 == y {
            let start = coord.0.min(x);
            let end = coord.0.max(x);

            for x in start + 1..end {
                if field.at(x, y).is_asteroid() {
                    blocked = true;
                    break;
                }
            }
        } else {
            // y = mx + b
            let slope = (coord.1 as f64 - y as f64) / (coord.0 as f64 - x as f64);
            let b = y as f64 - (slope * x as f64);

            let point = |x| slope * (x as f64) + b;

            for coord2 in field.asteroid_points() {
                let outside_segment = || {
                    let (x_min, x_max) = (x.min(coord.0), x.max(coord.0));
                    let (y_min, y_max) = (y.min(coord.1), y.max(coord.1));

                    (coord2.0 > x_max || coord2.0 < x_min) && (coord2.1 > y_max || coord2.1 < y_min)
                };

                if (coord2.1 as f64 - point(coord2.0)).abs() < 0.0001
                    && coord2 != coord
                    && coord2 != (x, y)
                    && !outside_segment()
                {
                    blocked = true;
                    break;
                }
            }
        }

        if !blocked {
            count += 1;
        }
    }

    count
}

#[test]
fn part1_tests() {
    test_input(
        ".#..#
         .....
         #####
         ....#
         ...##",
        (3, 4, 8),
    );

    test_input(
        "......#.#.
        #..#.#....
        ..#######.
        .#.#.###..
        .#..#.....
        ..#....#.#
        #..#....#.
        .##.#..###
        ##...#..#.
        .#....####",
        (5, 8, 33),
    );

    test_input(
        "#.#...#.#.
        .###....#.
        .#....#...
        ##.#.#.#.#
        ....#.#.#.
        .##..###.#
        ..#...##..
        ..##....##
        ......#...
        .####.###.",
        (1, 2, 35),
    );

    test_input(
        ".#..#..###
        ####.###.#
        ....###.#.
        ..###.##.#
        ##.##.#.#.
        ....###..#
        ..#.#..#.#
        #..#.#.###
        .##...##.#
        .....#.#..",
        (6, 3, 41),
    );

    test_input(
        ".#..##.###...#######
        ##.############..##.
        .#.######.########.#
        .###.#######.####.#.
        #####.##.#.##.###.##
        ..#####..#.#########
        ####################
        #.####....###.#.#.##
        ##.#################
        #####.##.###..####..
        ..######..##.#######
        ####.##.####...##..#
        .#####..#.######.###
        ##...#.##########...
        #.##########.#######
        .####.#.###.###.#.##
        ....##.##.###..#####
        .#.#.###########.###
        #.#.#.#####.####.###
        ###.##.####.##.#..##",
        (11, 13, 210),
    );
}

#[cfg(test)]
fn test_input(input: &str, expected: (usize, usize, usize)) {
    assert_eq!(part1(&gen(input)), Answer((expected.0, expected.1), expected.2));
}
