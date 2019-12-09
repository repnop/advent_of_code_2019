use aoc_runner_derive::aoc;

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;
    const LAYER_SPLIT: usize = WIDTH * HEIGHT;

    let layer = {
        let mut offset = 0;
        let mut lowest_offset = usize::max_value();
        let mut lowest_count = usize::max_value();

        while offset < input.len() {
            let layer1 = &input[offset..][..LAYER_SPLIT];
            let count = layer1.chars().filter(|&c| c == '0').count();

            if count < lowest_count {
                lowest_count = count;
                lowest_offset = offset;
            }

            offset += LAYER_SPLIT;
        }

        &input[lowest_offset..][..LAYER_SPLIT]
    };

    let (ones, twos) =
        layer.chars().filter(|&c| c == '1' || c == '2').fold((0, 0), |(mut o, mut t), c| {
            if c == '1' {
                o += 1;
            } else {
                t += 1;
            }

            (o, t)
        });

    ones * twos
}

#[aoc(day8, part2)]
fn part2(input: &str) -> String {
    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;
    const LAYER_SPLIT: usize = WIDTH * HEIGHT;

    let mut image = Vec::with_capacity(LAYER_SPLIT);

    image.extend_from_slice(&input.as_bytes()[..LAYER_SPLIT]);

    assert_eq!(image.len(), LAYER_SPLIT);

    let mut offset = LAYER_SPLIT;

    while offset < input.len() {
        let slice = &input.as_bytes()[offset..][..LAYER_SPLIT];

        for (new_pixel, old_pixel) in slice.iter().copied().zip(image.iter_mut()) {
            if new_pixel != b'2' && *old_pixel == b'2' {
                *old_pixel = new_pixel;
            }
        }

        offset += LAYER_SPLIT;
    }

    let mut output_string = String::with_capacity(LAYER_SPLIT);
    output_string.push('\n');

    for (i, pixel) in image.iter().copied().enumerate() {
        let c = match pixel {
            b'0' => '█',
            b'1' => ' ',
            //b'2' => ' ',
            _ => unreachable!(),
        };

        output_string.push(c);

        if (i + 1) % WIDTH == 0 && i != 0 {
            output_string.push('\n');
            dbg!();
        }
    }

    output_string
}

#[test]
fn bar() {
    println!("{}", part2(include_str!("../input/2019/day8.txt").trim()));
}
