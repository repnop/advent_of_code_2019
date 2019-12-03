use aoc_runner_derive::{aoc, aoc_generator};

pub struct Instruction {
    opcode: InstructionKind,
    op1: usize,
    op2: usize,
    dst: usize,
}

pub enum InstructionKind {
    Add,
    Mul,
    Halt,
}

impl Instruction {
    pub fn new(opcode: InstructionKind, op1: usize, op2: usize, dst: usize) -> Self {
        Self {
            opcode,
            op1,
            op2,
            dst,
        }
    }

    pub fn halt() -> Self {
        Self::new(InstructionKind::Halt, 0, 0, 0)
    }

    fn decode(bytes: &[usize]) -> Option<Instruction> {
        use InstructionKind::*;

        match bytes[0] {
            1 => Some(Instruction::new(Add, bytes[1], bytes[2], bytes[3])),
            2 => Some(Instruction::new(Mul, bytes[1], bytes[2], bytes[3])),
            99 => Some(Instruction::halt()),
            _ => None,
        }
    }
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap()
}

#[aoc(day2, part1)]
fn part1(bytes: &[usize]) -> usize {
    let mut bytes = bytes.to_vec();

    // before running the program, replace position 1 with the value 12 and
    // replace position 2 with the value 2
    bytes[1] = 12;
    bytes[2] = 2;

    let mut start = 0;

    loop {
        let inst = Instruction::decode(&bytes[start..]).unwrap();
        start += 4;

        match inst.opcode {
            InstructionKind::Add => bytes[inst.dst] = bytes[inst.op1] + bytes[inst.op2],
            InstructionKind::Mul => bytes[inst.dst] = bytes[inst.op1] * bytes[inst.op2],
            InstructionKind::Halt => break,
        }
    }

    bytes[0]
}

#[aoc(day2, part2)]
fn part2(bytes: &[usize]) -> String {
    let mut mbytes = Vec::with_capacity(bytes.len());
    let desired = 19690720;

    for noun in 0..=99 {
        for verb in 0..=99 {
            mbytes.clear();
            mbytes.extend_from_slice(bytes);

            mbytes[1] = noun;
            mbytes[2] = verb;

            let mut start = 0;

            loop {
                let inst = Instruction::decode(&mbytes[start..]).unwrap();
                start += 4;

                match inst.opcode {
                    InstructionKind::Add => mbytes[inst.dst] = mbytes[inst.op1] + mbytes[inst.op2],
                    InstructionKind::Mul => mbytes[inst.dst] = mbytes[inst.op1] * mbytes[inst.op2],
                    InstructionKind::Halt => break,
                }
            }

            if mbytes[0] == desired {
                return format!(
                    "noun: {}, verb: {}, 100 * noun + verb = {}",
                    noun,
                    verb,
                    100 * noun + verb
                );
            }
        }
    }

    unreachable!()
}
