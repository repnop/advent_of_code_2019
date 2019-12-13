use crate::intcode::*;
use aoc_runner_derive::aoc;
use crossbeam_channel::{bounded, unbounded};
use std::collections::HashMap;

fn gen(input: &str) -> Vec<isize> {
    input.split(',').map(str::parse).collect::<Result<_, _>>().unwrap()
}

const BLOCK: isize = 2;
const PADDLE: isize = 3;
const BALL: isize = 4;

#[aoc(day13, part1)]
fn part1(input: &str) -> usize {
    let (intcode_input, intcode_output) = unbounded();
    let program = gen(input);

    let cpu_thread = std::thread::spawn(move || {
        let mut machine = IntcodeMachine::new(&program, empty(), intcode_input);

        machine.run();
    });

    let drawer_thread = std::thread::spawn(move || {
        let mut tiles = HashMap::new();

        while let Ok(x) = intcode_output.recv() {
            let y = intcode_output.recv().unwrap();
            let kind = intcode_output.recv().unwrap();

            tiles.insert((x, y), kind);
        }

        tiles.values().filter(|kind| **kind == BLOCK).count()
    });

    cpu_thread.join().unwrap();
    drawer_thread.join().unwrap()
}

#[aoc(day13, part2)]
fn part2(input: &str) -> isize {
    let (intcode_tx, intcode_output) = bounded(1);
    let (intcode_command, intcode_input) = bounded(1);
    let mut program = gen(input);

    let cpu_thread = std::thread::spawn(move || {
        program[0] = 2;
        let mut machine = IntcodeMachine::new(&program, intcode_input.into_iter(), intcode_tx);

        machine.run();
    });

    let drawer_thread = std::thread::spawn(move || {
        use std::cmp::Ordering;

        let mut tiles: HashMap<(isize, isize), isize> = HashMap::new();
        let mut score = 0;
        let (mut paddle_x, mut paddle_y) = (0, 0);
        let (mut ball_delta, mut ball_x, mut ball_y) = (0, 0, 0);
        let mut drawn = false;

        print!("\x1B[2J\x1B[1;1H\x1B[?25l");
        use std::io::Write;
        std::io::stdout().lock().flush().unwrap();

        while let Ok(x) = intcode_output.recv() {
            let y = intcode_output.recv().unwrap();
            let third = intcode_output.recv().unwrap();

            if (x, y) == (-1, 0) {
                score = third;
            } else {
                if third == BALL {
                    ball_delta = x - ball_x;
                    ball_x = x;
                    ball_y = y;
                } else if third == PADDLE {
                    paddle_x = x;
                    paddle_y = y;
                }

                let tile = match third {
                    0 => ' ',
                    1 => '█',
                    2 => '▒',
                    3 => '─',
                    4 => '0',
                    _ => unreachable!(),
                };

                tiles.insert((x, y), third);

                if !drawn {
                    drawn = tiles.get(&(41, 22)).is_some();
                }

                print!("\x1B[{};{}H", y + 1, x + 1);
                use std::io::Write;
                std::io::stdout().lock().flush().unwrap();
                print!("{}", tile);
                std::io::stdout().lock().flush().unwrap();
            }

            if drawn {
                let cmd = match (ball_x.cmp(&paddle_x), ball_delta, paddle_y - ball_y == 1) {
                    (Ordering::Less, d, _) if d.is_negative() => -1,
                    (Ordering::Equal, d, _) if d.is_negative() => -1,
                    (Ordering::Equal, _, _) => 1,
                    (Ordering::Greater, d, _) if d.is_positive() => 1,
                    _ => 0,
                };

                print!(
                    "\x1B[25;1H                              \r{:?}, {:?}, {:?}, {:?}",
                    ball_x.cmp(&paddle_x),
                    ball_delta,
                    paddle_y - ball_y == 1,
                    cmd
                );
                std::io::stdout().lock().flush().unwrap();

                if drawn {
                    std::thread::sleep(std::time::Duration::from_millis(64));
                }

                let _ = intcode_command.try_send(cmd);
            }
        }

        print!("\x1B[?25h");
        std::io::stdout().lock().flush().unwrap();

        score
    });

    cpu_thread.join().unwrap();
    drawer_thread.join().unwrap()
}

#[cfg(feature = "termion")]
macro_rules! print_flush {
    ($s:ident, $($ts:tt)*) => {{
        write!($s, $($ts)*);
        $s.flush().unwrap();
    }}
}

#[cfg(feature = "termion")]
#[aoc(day13, part2, "actually play it")]
fn part2(input: &str) -> isize {
    let (intcode_tx, intcode_output) = unbounded();
    let (kill_tx, kill_rx) = unbounded();
    let mut program = gen(input);

    let cpu_thread = std::thread::spawn(move || {
        use std::io::{BufRead, Write};
        use termion::{event::Key, input::TermRead};

        let stdin = std::io::stdin();
        program[0] = 2;
        let mut machine = IntcodeMachine::new(
            &program,
            stdin.keys().filter_map(|k| {
                Some(match k.ok()? {
                    Key::Right => 1,
                    Key::Left => -1,
                    Key::Ctrl('c') => {
                        kill_tx.send(());
                        std::thread::sleep(std::time::Duration::from_millis(16));
                        std::process::exit(0)
                    }
                    _ => 0,
                })
            }),
            intcode_tx,
        );

        machine.run();
    });

    let drawer_thread = std::thread::spawn(move || -> Result<isize, ()> {
        use std::io::Write;
        use termion::raw::IntoRawMode;

        let mut stdout = std::io::stdout().into_raw_mode().unwrap();

        let mut tiles: HashMap<(isize, isize), isize> = HashMap::new();
        let mut score = 0isize;
        let mut term_size = termion::terminal_size().map_err(drop)?;

        print_flush!(stdout, "{}", termion::clear::All);

        let mut at_input = false;
        let mut after_input = true;

        loop {
            if let Ok(_) = kill_rx.recv_timeout(std::time::Duration::from_millis(1)) {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(4));

            if after_input {
                print_flush!(
                    stdout,
                    "{}{}",
                    termion::cursor::Goto(1, 1),
                    termion::clear::CurrentLine
                );
                print_flush!(
                    stdout,
                    "{}Score: {}\n\r---------------",
                    termion::cursor::Goto(1, 1),
                    score
                );
                print_flush!(stdout, "{}", termion::cursor::Goto(1, term_size.1));
                after_input = false;
            }

            //if after_input {
            //    print_flush!("{}", termion::clear::All);
            //    print_flush!("{}{}", termion::cursor::Goto(1, 1), termion::clear::CurrentLine);
            //    print_flush!("{}Score: {}\n---------------", termion::cursor::Goto(1, 1), score);
            //    print_flush!("{}", termion::cursor::Goto(1, term_size.1));
            //
            //    for (k, v) in &tiles {
            //        print_flush!("{}", termion::cursor::Goto(1 + k.0 as u16, 3 + k.1 as u16));
            //        print_flush!(
            //            "{}",
            //            match v {
            //                0 => ' ',
            //                1 => '█',
            //                2 => '▒',
            //                3 => '─',
            //                4 => '0',
            //                _ => unreachable!(),
            //            }
            //        );
            //        std::thread::sleep(std::time::Duration::from_micros(5));
            //    }
            //
            //    after_input = false;
            //}

            let x = match intcode_output.recv_timeout(std::time::Duration::from_millis(1)) {
                Ok(x) => x,
                Err(RecvTimeoutError::Disconnected) => break,
                Err(RecvTimeoutError::Timeout) => {
                    print_flush!(stdout, "{}", termion::cursor::Goto(1, term_size.1 - 1));
                    print_flush!(stdout, "{}", termion::clear::CurrentLine);
                    print_flush!(stdout, "{}", termion::cursor::Goto(1, term_size.1 - 1));
                    at_input = true;
                    continue;
                }
            };

            if at_input {
                at_input = false;
                after_input = true;
            }

            let y = intcode_output.recv().map_err(drop)?;
            let third = intcode_output.recv().map_err(drop)?;

            if (x, y) == (-1, 0) {
                score = third;
            } else {
                print_flush!(stdout, "{}", termion::cursor::Goto(1 + x as u16, 3 + y as u16));
                print_flush!(
                    stdout,
                    "{}",
                    match third {
                        0 => ' ',
                        1 => '█',
                        2 => '▒',
                        3 => '─',
                        4 => '0',
                        _ => unreachable!(),
                    }
                );
                tiles.insert((x, y), third);
            }
        }

        Ok(score)
    });

    cpu_thread.join().unwrap();
    drawer_thread.join().unwrap().unwrap()
}
