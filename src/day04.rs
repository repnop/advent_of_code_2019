use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn get_range(input: &str) -> (u32, u32) {
    let mut strs = input.split('-');

    (
        strs.next().unwrap().parse().unwrap(),
        strs.next().unwrap().parse().unwrap(),
    )
}

fn make_iter<'a>(digits: &'a [u8; 6]) -> impl Iterator<Item = u8> + Clone + 'a {
    digits.iter().copied()
}

fn get_digits(mut n: u32) -> [u8; 6] {
    let mut digits = [0u8; 6];
    let mut idx = 5;

    loop {
        let m = n % 10;
        n /= 10;

        digits[idx] = m as u8;
        idx = idx.wrapping_sub(1);

        if n == 0 {
            break;
        }
    }

    digits
}

#[aoc(day4, part1)]
fn part1(&(start, stop): &(u32, u32)) -> u32 {
    let mut count = 0;

    for i in start..=stop {
        let digits = get_digits(i);
        let iter = make_iter(&digits);

        if !iter
            .clone()
            .zip(iter.clone().skip(1))
            .all(|(n1, n2)| n1 <= n2)
        {
            continue;
        }

        if !iter
            .clone()
            .zip(iter.clone().skip(1))
            .any(|(n1, n2)| n1 == n2)
        {
            continue;
        }

        count += 1;
    }

    count
}

#[aoc(day4, part2)]
fn part2(&(start, stop): &(u32, u32)) -> u32 {
    let mut count = 0;

    for i in start..=stop {
        let digits = get_digits(i);
        let iter = make_iter(&digits);

        if !iter
            .clone()
            .zip(iter.clone().skip(1))
            .all(|(n1, n2)| n1 <= n2)
        {
            continue;
        }

        if !iter
            .clone()
            .zip(iter.clone().skip(1))
            .any(|(n1, n2)| n1 == n2)
        {
            continue;
        }

        if !at_least_one_single_double(iter) {
            continue;
        }

        count += 1;
    }

    count
}

fn at_least_one_single_double(mut i: impl Iterator<Item = u8>) -> bool {
    let mut current_digit = i.next().unwrap();
    let mut count = 1;

    for digit in i {
        let diff = digit != current_digit;

        if diff && count == 2 {
            return true;
        } else if diff {
            current_digit = digit;
            count = 1;
        } else {
            count += 1;
        }
    }

    count == 2
}

#[test]
fn single_doubles() {
    assert!(at_least_one_single_double(make_iter(&get_digits(112233))));
    assert!(!at_least_one_single_double(make_iter(&get_digits(123444))));
    assert!(at_least_one_single_double(make_iter(&get_digits(111122))));
}
