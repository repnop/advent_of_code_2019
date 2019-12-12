use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn sum(self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Moon {
    position: Vec3,
    velocity: Vec3,
}

impl Moon {
    fn new(position: Vec3) -> Self {
        Self { position, velocity: Vec3::default() }
    }

    fn apply_velocity(&mut self) {
        self.position += self.velocity;
    }

    fn total_energy(self) -> i32 {
        self.position.sum() * self.velocity.sum()
    }

    fn gravity_delta(self, rhs: Self) -> Vec3 {
        let cmp = |a: i32, b: i32| {
            use std::cmp::Ordering;
            match a.cmp(&b) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            }
        };

        Vec3::new(
            cmp(self.position.x, rhs.position.x),
            cmp(self.position.y, rhs.position.y),
            cmp(self.position.z, rhs.position.z),
        )
    }
}

impl std::fmt::Debug for Moon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "p=<x={:>3}, y={:>3}, z={:>3}>, vel=<x={:>3}, y={:>3}, z={:>3}>",
            self.position.x,
            self.position.y,
            self.position.z,
            self.velocity.x,
            self.velocity.y,
            self.velocity.z
        )
    }
}

fn gen(input: &str) -> [Moon; 4] {
    let i = input.lines().filter_map(|line| {
        let line = line.trim_matches(|c| c == '<' || c == '>');
        let mut line = line.split(',');

        let x = line.next()?.split('=').nth(1)?.parse().ok()?;
        let y = line.next()?.split('=').nth(1)?.parse().ok()?;
        let z = line.next()?.split('=').nth(1)?.parse().ok()?;

        Some(Moon::new(Vec3::new(x, y, z)))
    });

    unwrap_4(i)
}

#[aoc(day12, part1)]
fn part1(input: &str) -> i32 {
    let mut moons = gen(input);

    for _ in 0..1000 {
        let moon_0_delta = moons[0].gravity_delta(moons[1])
            + moons[0].gravity_delta(moons[2])
            + moons[0].gravity_delta(moons[3]);

        let moon_1_delta = moons[1].gravity_delta(moons[0])
            + moons[1].gravity_delta(moons[2])
            + moons[1].gravity_delta(moons[3]);

        let moon_2_delta = moons[2].gravity_delta(moons[0])
            + moons[2].gravity_delta(moons[1])
            + moons[2].gravity_delta(moons[3]);

        let moon_3_delta = moons[3].gravity_delta(moons[0])
            + moons[3].gravity_delta(moons[1])
            + moons[3].gravity_delta(moons[2]);

        moons[0].velocity += moon_0_delta;
        moons[0].apply_velocity();

        moons[1].velocity += moon_1_delta;
        moons[1].apply_velocity();

        moons[2].velocity += moon_2_delta;
        moons[2].apply_velocity();

        moons[3].velocity += moon_3_delta;
        moons[3].apply_velocity();
    }

    moons.iter().copied().fold(0, |acc, moon| acc + moon.total_energy())
}

fn unwrap_4<T>(mut i: impl Iterator<Item = T>) -> [T; 4] {
    [i.next().unwrap(), i.next().unwrap(), i.next().unwrap(), i.next().unwrap()]
}

/*
#[test]
fn first_test() {
    let input = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

    assert_eq!(part1(input), 179);
}*/
