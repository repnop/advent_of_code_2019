use enum_dispatch::enum_dispatch;

pub struct IntcodeMachine<'a, R: Iterator<Item = isize>, W: Sink<isize>> {
    data: &'a mut [isize],
    ip: usize,
    input: R,
    output: W,
}

impl<'a, R: Iterator<Item = isize>, W: Sink<isize>> IntcodeMachine<'a, R, W> {
    pub fn new(data: &'a mut [isize], input: R, output: W) -> Self {
        Self {
            data,
            ip: 0,
            input,
            output,
        }
    }

    pub fn run(&mut self) {
        loop {
            let (inst, offset) = Instructions::decode(&self.data[self.ip..]);
            self.ip += offset;

            #[cfg(debug_assertions)]
            eprintln!("{:?}", inst);

            if !inst.execute(self) {
                break;
            }
        }
    }

    pub fn data(&self) -> &[isize] {
        &self.data
    }
}

pub trait Sink<Item> {
    fn send(&mut self, item: Item);
}

impl Sink<isize> for std::io::Stdout {
    fn send(&mut self, item: isize) {
        use std::io::Write;

        write!(&mut self.lock(), "{}", item).unwrap();
    }
}

impl Sink<isize> for () {
    fn send(&mut self, _: isize) {}
}

impl Sink<isize> for [isize; 1] {
    fn send(&mut self, item: isize) {
        self[0] = item;
    }
}

impl Sink<isize> for isize {
    fn send(&mut self, item: isize) {
        *self = item;
    }
}

impl<T, U: Sink<T>> Sink<T> for &'_ mut U {
    fn send(&mut self, item: T) {
        (*self).send(item);
    }
}

pub type Position = usize;

#[derive(Clone, Copy, Debug)]
pub enum Operand {
    Immediate(isize),
    Position(usize),
}

impl Operand {
    fn from_parts(mode: Mode, value: isize) -> Self {
        match mode {
            Mode::Immediate => Operand::Immediate(value),
            Mode::Position => Operand::Position(value as usize),
        }
    }

    pub fn resolve(self, mem: &[isize]) -> isize {
        match self {
            Operand::Immediate(n) => n,
            Operand::Position(p) => mem[p],
        }
    }
}

#[derive(Debug)]
pub struct Add {
    dst: Position,
    op1: Operand,
    op2: Operand,
}

impl Add {
    pub fn new(dst: Position, op1: Operand, op2: Operand) -> Self {
        Self { dst, op1, op2 }
    }

    fn decode(opcode: Opcode, ints: &[isize]) -> (Instructions, usize) {
        let dst = ints[2] as Position;
        let op1 = Operand::from_parts(opcode.param1, ints[0]);
        let op2 = Operand::from_parts(opcode.param2, ints[1]);

        (Self::new(dst, op1, op2).into(), 4)
    }
}

impl Instruction for Add {
    fn execute<R: Iterator<Item = isize>, W: Sink<isize>>(
        &self,
        machine: &mut IntcodeMachine<R, W>,
    ) -> bool {
        let op1 = self.op1.resolve(&machine.data);
        let op2 = self.op2.resolve(&machine.data);

        machine.data[self.dst] = op1 + op2;

        true
    }
}

#[derive(Debug)]
pub struct Mul {
    dst: Position,
    op1: Operand,
    op2: Operand,
}

impl Mul {
    pub fn new(dst: Position, op1: Operand, op2: Operand) -> Self {
        Self { dst, op1, op2 }
    }

    fn decode(opcode: Opcode, ints: &[isize]) -> (Instructions, usize) {
        let dst = ints[2] as Position;
        let op1 = Operand::from_parts(opcode.param1, ints[0]);
        let op2 = Operand::from_parts(opcode.param2, ints[1]);

        (Self::new(dst, op1, op2).into(), 4)
    }
}

impl Instruction for Mul {
    fn execute<R: Iterator<Item = isize>, W: Sink<isize>>(
        &self,
        machine: &mut IntcodeMachine<R, W>,
    ) -> bool {
        let op1 = self.op1.resolve(&machine.data);
        let op2 = self.op2.resolve(&machine.data);

        machine.data[self.dst] = op1 * op2;

        true
    }
}

#[derive(Debug)]
pub struct Halt;

impl Halt {
    pub fn new() -> Self {
        Self
    }
}

impl Instruction for Halt {
    fn execute<R: Iterator<Item = isize>, W: Sink<isize>>(
        &self,
        _: &mut IntcodeMachine<R, W>,
    ) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct Input {
    operand: Position,
}

impl Input {
    pub fn new(operand: Position) -> Self {
        Self { operand }
    }

    fn decode(_: Opcode, ints: &[isize]) -> (Instructions, usize) {
        let operand = ints[0] as Position;

        (Self::new(operand).into(), 2)
    }
}

impl Instruction for Input {
    fn execute<R: Iterator<Item = isize>, W: Sink<isize>>(
        &self,
        machine: &mut IntcodeMachine<R, W>,
    ) -> bool {
        let inp = machine.input.next().unwrap();

        machine.data[self.operand] = inp;

        true
    }
}

#[derive(Debug)]
pub struct Output {
    operand: Operand,
}

impl Output {
    pub fn new(operand: Operand) -> Self {
        Self { operand }
    }

    fn decode(opcode: Opcode, ints: &[isize]) -> (Instructions, usize) {
        let operand = Operand::from_parts(opcode.param1, ints[0]);

        (Self::new(operand).into(), 2)
    }
}

impl Instruction for Output {
    fn execute<R: Iterator<Item = isize>, W: Sink<isize>>(
        &self,
        machine: &mut IntcodeMachine<R, W>,
    ) -> bool {
        let val = self.operand.resolve(&machine.data);

        machine.output.send(val);

        true
    }
}

#[derive(Debug)]
pub struct JumpIfTrue {
    test: Operand,
    jump_to: Operand,
}

impl JumpIfTrue {
    pub fn new(test: Operand, jump_to: Operand) -> Self {
        Self { test, jump_to }
    }

    fn decode(opcode: Opcode, ints: &[isize]) -> (Instructions, usize) {
        let test = Operand::from_parts(opcode.param1, ints[0]);
        let jump_to = Operand::from_parts(opcode.param2, ints[1]);

        (Self::new(test, jump_to).into(), 3)
    }
}

impl Instruction for JumpIfTrue {
    fn execute<R: Iterator<Item = isize>, W: Sink<isize>>(
        &self,
        machine: &mut IntcodeMachine<R, W>,
    ) -> bool {
        if self.test.resolve(&machine.data) != 0 {
            machine.ip = self.jump_to.resolve(&machine.data) as Position;
        }

        true
    }
}

#[derive(Debug)]
pub struct JumpIfFalse {
    test: Operand,
    jump_to: Operand,
}

impl JumpIfFalse {
    pub fn new(test: Operand, jump_to: Operand) -> Self {
        Self { test, jump_to }
    }

    fn decode(opcode: Opcode, ints: &[isize]) -> (Instructions, usize) {
        let test = Operand::from_parts(opcode.param1, ints[0]);
        let jump_to = Operand::from_parts(opcode.param2, ints[1]);

        (Self::new(test, jump_to).into(), 3)
    }
}

impl Instruction for JumpIfFalse {
    fn execute<R: Iterator<Item = isize>, W: Sink<isize>>(
        &self,
        machine: &mut IntcodeMachine<R, W>,
    ) -> bool {
        if self.test.resolve(&machine.data) == 0 {
            machine.ip = self.jump_to.resolve(&machine.data) as Position;
        }

        true
    }
}

#[derive(Debug)]
pub struct LessThan {
    op1: Operand,
    op2: Operand,
    dst: Position,
}

impl LessThan {
    pub fn new(op1: Operand, op2: Operand, dst: Position) -> Self {
        Self { op1, op2, dst }
    }

    fn decode(opcode: Opcode, ints: &[isize]) -> (Instructions, usize) {
        let op1 = Operand::from_parts(opcode.param1, ints[0]);
        let op2 = Operand::from_parts(opcode.param2, ints[1]);
        let dst = ints[2] as Position;

        (Self::new(op1, op2, dst).into(), 4)
    }
}

impl Instruction for LessThan {
    fn execute<R: Iterator<Item = isize>, W: Sink<isize>>(
        &self,
        machine: &mut IntcodeMachine<R, W>,
    ) -> bool {
        if self.op1.resolve(&machine.data) < self.op2.resolve(&machine.data) {
            machine.data[self.dst] = 1;
        } else {
            machine.data[self.dst] = 0;
        }

        true
    }
}

#[derive(Debug)]
pub struct EqualTo {
    op1: Operand,
    op2: Operand,
    dst: Position,
}

impl EqualTo {
    pub fn new(op1: Operand, op2: Operand, dst: Position) -> Self {
        Self { op1, op2, dst }
    }

    fn decode(opcode: Opcode, ints: &[isize]) -> (Instructions, usize) {
        let op1 = Operand::from_parts(opcode.param1, ints[0]);
        let op2 = Operand::from_parts(opcode.param2, ints[1]);
        let dst = ints[2] as Position;

        (Self::new(op1, op2, dst).into(), 4)
    }
}

impl Instruction for EqualTo {
    fn execute<R: Iterator<Item = isize>, W: Sink<isize>>(
        &self,
        machine: &mut IntcodeMachine<R, W>,
    ) -> bool {
        if self.op1.resolve(&machine.data) == self.op2.resolve(&machine.data) {
            machine.data[self.dst] = 1;
        } else {
            machine.data[self.dst] = 0;
        }

        true
    }
}

#[enum_dispatch(Instructions)]
pub trait Instruction {
    fn execute<R: Iterator<Item = isize>, W: Sink<isize>>(
        &self,
        machine: &mut IntcodeMachine<R, W>,
    ) -> bool;
}

#[enum_dispatch]
#[derive(Debug)]
pub enum Instructions {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    EqualTo,
    Halt,
}

impl Instructions {
    pub fn decode(ints: &[isize]) -> (Self, usize) {
        const ADD_OP: usize = 1;
        const MUL_OP: usize = 2;
        const INP_OP: usize = 3;
        const OUT_OP: usize = 4;
        const JIT_OP: usize = 5;
        const JIF_OP: usize = 6;
        const LST_OP: usize = 7;
        const EQU_OP: usize = 8;
        const HALT_OP: usize = 99;

        let opcode = Opcode::from(ints[0]);

        match opcode.opcode {
            ADD_OP => Add::decode(opcode, &ints[1..]),
            MUL_OP => Mul::decode(opcode, &ints[1..]),
            INP_OP => Input::decode(opcode, &ints[1..]),
            OUT_OP => Output::decode(opcode, &ints[1..]),
            JIT_OP => JumpIfTrue::decode(opcode, &ints[1..]),
            JIF_OP => JumpIfFalse::decode(opcode, &ints[1..]),
            LST_OP => LessThan::decode(opcode, &ints[1..]),
            EQU_OP => EqualTo::decode(opcode, &ints[1..]),
            HALT_OP => (Halt::new().into(), 1),
            n => panic!("Invalid opcode: {}, {}", n, ints[0]),
        }
    }
}

enum Mode {
    Position = 0,
    Immediate = 1,
}

impl From<usize> for Mode {
    fn from(i: usize) -> Self {
        match i {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => panic!("Invalid mode"),
        }
    }
}

struct Opcode {
    opcode: usize,
    param1: Mode,
    param2: Mode,
    #[allow(unused)]
    param3: Mode,
}

impl From<isize> for Opcode {
    fn from(i: isize) -> Self {
        let mut i = i as usize;
        let opcode = i % 100;

        i /= 100;
        let param1 = (i % 10).into();

        i /= 10;
        let param2 = (i % 10).into();

        i /= 10;
        let param3 = (i % 10).into();

        Self {
            opcode,
            param1,
            param2,
            param3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::once;

    #[test]
    fn conditionals() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let mut input: Vec<isize> = input
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()
            .unwrap();

        let mut output = 0isize;
        let mut machine = IntcodeMachine::new(&mut input, once(7), &mut output);
        machine.run();
        assert_eq!(output, 999);

        let mut output = 0isize;
        let mut machine = IntcodeMachine::new(&mut input, once(8), &mut output);
        machine.run();
        assert_eq!(output, 1000);

        let mut output = 0isize;
        let mut machine = IntcodeMachine::new(&mut input, once(9), &mut output);
        machine.run();
        assert_eq!(output, 1001);
    }
}
