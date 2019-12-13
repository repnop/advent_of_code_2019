use enum_dispatch::enum_dispatch;

macro_rules! debug {
    ($($ts:tt)*) => {{
        #[cfg(debug_assertions)]
        eprintln!($($ts)*);
    }}
}

pub fn empty() -> impl Iterator<Item = isize> {
    None.into_iter()
}

pub struct IntcodeMachine<R: Iterator<Item = isize>, W: Sink<isize>> {
    data: Vec<isize>,
    ip: usize,
    relative_base: isize,
    input: R,
    output: W,
    running: bool,
}

impl<R: Iterator<Item = isize>, W: Sink<isize>> IntcodeMachine<R, W> {
    pub fn new(program: &[isize], input: R, output: W) -> Self {
        let mut data = vec![0; 4096];
        data[..program.len()].copy_from_slice(&program);

        Self { data, ip: 0, relative_base: 0, input, output, running: true }
    }

    pub fn run(&mut self) {
        while self.running {
            let inst = Instructions::decode(
                &self.data[self.ip..],
                #[cfg(debug_assertions)]
                self.ip,
            );
            self.ip += inst.size();
            inst.execute(self);
        }
    }

    pub fn data(&self) -> &[isize] {
        &self.data
    }
}

pub trait Sink<Item> {
    fn send(&mut self, item: Item);
}

impl<U: Clone, T: Sink<U>, V: Sink<U>> Sink<U> for (T, V) {
    fn send(&mut self, item: U) {
        self.0.send(item.clone());
        self.1.send(item);
    }
}

impl Sink<isize> for crossbeam_channel::Sender<isize> {
    fn send(&mut self, item: isize) {
        crossbeam_channel::Sender::send(&*self, item).unwrap();
    }
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

impl<T> Sink<T> for Vec<T> {
    fn send(&mut self, item: T) {
        self.push(item);
    }
}

impl<T, U: Sink<T>> Sink<T> for &'_ mut U {
    fn send(&mut self, item: T) {
        (*self).send(item);
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Operand {
    Immediate(isize),
    Position(usize),
    Relative(isize),
}

impl Operand {
    fn from_parts(mode: Mode, value: isize) -> Self {
        match mode {
            Mode::Immediate => Operand::Immediate(value),
            Mode::Position => Operand::Position(value as usize),
            Mode::Relative => Operand::Relative(value),
        }
    }

    pub fn resolve<R: Iterator<Item = isize>, W: Sink<isize>>(
        self,
        machine: &IntcodeMachine<R, W>,
    ) -> isize {
        match self {
            Operand::Immediate(n) => n,
            Operand::Position(p) => machine.data[p],
            Operand::Relative(r) => machine.data[(machine.relative_base + r) as usize],
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Destination {
    Position(usize),
    Relative(usize),
}

impl Destination {
    fn from_parts(mode: Mode, value: isize) -> Self {
        match mode {
            Mode::Immediate => panic!("Destinations can't be immediate mode"),
            Mode::Position => Destination::Position(value as usize),
            Mode::Relative => Destination::Relative(value as usize),
        }
    }

    pub fn resolve<R: Iterator<Item = isize>, W: Sink<isize>>(
        self,
        machine: &IntcodeMachine<R, W>,
    ) -> usize {
        match self {
            Destination::Position(n) => n,
            Destination::Relative(r) => (r as isize + machine.relative_base) as usize,
        }
    }
}

#[derive(Debug)]
pub struct Add {
    dst: Destination,
    op1: Operand,
    op2: Operand,
}

impl Add {
    pub fn new(dst: Destination, op1: Operand, op2: Operand) -> Self {
        Self { dst, op1, op2 }
    }

    fn decode(opcode: Opcode, ints: &[isize]) -> Instructions {
        let op1 = Operand::from_parts(opcode.param1, ints[0]);
        let op2 = Operand::from_parts(opcode.param2, ints[1]);
        let dst = Destination::from_parts(opcode.param3, ints[2]);

        Self::new(dst, op1, op2).into()
    }
}

impl Instruction for Add {
    fn execute<R: Iterator<Item = isize>, W: Sink<isize>>(
        &self,
        machine: &mut IntcodeMachine<R, W>,
    ) {
        let op1 = self.op1.resolve(&machine);
        let op2 = self.op2.resolve(&machine);
        let dst = self.dst.resolve(&machine);

        debug!("Add: memory[{}] = {} + {}", dst, op1, op2);

        machine.data[dst] = op1 + op2;
    }

    fn size(&self) -> usize {
        4
    }
}

#[derive(Debug)]
pub struct Mul {
    dst: Destination,
    op1: Operand,
    op2: Operand,
}

impl Mul {
    pub fn new(dst: Destination, op1: Operand, op2: Operand) -> Self {
        Self { dst, op1, op2 }
    }

    fn decode(opcode: Opcode, ints: &[isize]) -> Instructions {
        let op1 = Operand::from_parts(opcode.param1, ints[0]);
        let op2 = Operand::from_parts(opcode.param2, ints[1]);
        let dst = Destination::from_parts(opcode.param3, ints[2]);

        Self::new(dst, op1, op2).into()
    }
}

impl Instruction for Mul {
    fn execute<R: Iterator<Item = isize>, W: Sink<isize>>(
        &self,
        machine: &mut IntcodeMachine<R, W>,
    ) {
        let op1 = self.op1.resolve(&machine);
        let op2 = self.op2.resolve(&machine);
        let dst = self.dst.resolve(&machine);

        debug!("Mul: memory[{}] = {} * {}", dst, op1, op2);

        machine.data[dst] = op1 * op2;
    }

    fn size(&self) -> usize {
        4
    }
}

#[derive(Debug, Default)]
pub struct Halt;

impl Halt {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Instruction for Halt {
    fn execute<R: Iterator<Item = isize>, W: Sink<isize>>(
        &self,
        machine: &mut IntcodeMachine<R, W>,
    ) {
        machine.running = false;
    }

    fn size(&self) -> usize {
        1
    }
}

#[derive(Debug)]
pub struct Input {
    operand: Destination,
}

impl Input {
    pub fn new(operand: Destination) -> Self {
        Self { operand }
    }

    fn decode(opcode: Opcode, ints: &[isize]) -> Instructions {
        let operand = Destination::from_parts(opcode.param1, ints[0]);

        Self::new(operand).into()
    }
}

impl Instruction for Input {
    fn execute<R: Iterator<Item = isize>, W: Sink<isize>>(
        &self,
        machine: &mut IntcodeMachine<R, W>,
    ) {
        let inp = machine.input.next().unwrap();
        let dst = self.operand.resolve(&machine);

        debug!("Input: memory[{}] = {}", dst, inp);

        machine.data[dst] = inp;
    }

    fn size(&self) -> usize {
        2
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

    fn decode(opcode: Opcode, ints: &[isize]) -> Instructions {
        let operand = Operand::from_parts(opcode.param1, ints[0]);

        Self::new(operand).into()
    }
}

impl Instruction for Output {
    fn execute<R: Iterator<Item = isize>, W: Sink<isize>>(
        &self,
        machine: &mut IntcodeMachine<R, W>,
    ) {
        let val = self.operand.resolve(&machine);

        debug!("Output: sending {}", val);

        machine.output.send(val);
    }

    fn size(&self) -> usize {
        2
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

    fn decode(opcode: Opcode, ints: &[isize]) -> Instructions {
        let test = Operand::from_parts(opcode.param1, ints[0]);
        let jump_to = Operand::from_parts(opcode.param2, ints[1]);

        Self::new(test, jump_to).into()
    }
}

impl Instruction for JumpIfTrue {
    fn execute<R: Iterator<Item = isize>, W: Sink<isize>>(
        &self,
        machine: &mut IntcodeMachine<R, W>,
    ) {
        if self.test.resolve(&machine) != 0 {
            let jump_to = self.jump_to.resolve(&machine) as usize;
            debug!("JumpIfTrue: ip = {}", jump_to);
            machine.ip = jump_to;
        }
    }

    fn size(&self) -> usize {
        3
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

    fn decode(opcode: Opcode, ints: &[isize]) -> Instructions {
        let test = Operand::from_parts(opcode.param1, ints[0]);
        let jump_to = Operand::from_parts(opcode.param2, ints[1]);

        Self::new(test, jump_to).into()
    }
}

impl Instruction for JumpIfFalse {
    fn execute<R: Iterator<Item = isize>, W: Sink<isize>>(
        &self,
        machine: &mut IntcodeMachine<R, W>,
    ) {
        if self.test.resolve(&machine) == 0 {
            let jump_to = self.jump_to.resolve(&machine) as usize;
            debug!("JumpIfTrue: ip = {}", jump_to);
            machine.ip = jump_to;
        }
    }

    fn size(&self) -> usize {
        3
    }
}

#[derive(Debug)]
pub struct LessThan {
    dst: Destination,
    op1: Operand,
    op2: Operand,
}

impl LessThan {
    pub fn new(dst: Destination, op1: Operand, op2: Operand) -> Self {
        Self { dst, op1, op2 }
    }

    fn decode(opcode: Opcode, ints: &[isize]) -> Instructions {
        let op1 = Operand::from_parts(opcode.param1, ints[0]);
        let op2 = Operand::from_parts(opcode.param2, ints[1]);
        let dst = Destination::from_parts(opcode.param3, ints[2]);

        Self::new(dst, op1, op2).into()
    }
}

impl Instruction for LessThan {
    fn execute<R: Iterator<Item = isize>, W: Sink<isize>>(
        &self,
        machine: &mut IntcodeMachine<R, W>,
    ) {
        let dst = self.dst.resolve(&machine);
        let op1 = self.op1.resolve(&machine);
        let op2 = self.op2.resolve(&machine);

        if op1 < op2 {
            debug!("LessThan: memory[{}] = 1 ({} < {})", dst, op1, op2);
            machine.data[dst] = 1;
        } else {
            debug!("LessThan: memory[{}] = 0 ({} >= {})", dst, op1, op2);
            machine.data[dst] = 0;
        }
    }

    fn size(&self) -> usize {
        4
    }
}

#[derive(Debug)]
pub struct EqualTo {
    dst: Destination,
    op1: Operand,
    op2: Operand,
}

impl EqualTo {
    pub fn new(dst: Destination, op1: Operand, op2: Operand) -> Self {
        Self { dst, op1, op2 }
    }

    fn decode(opcode: Opcode, ints: &[isize]) -> Instructions {
        let op1 = Operand::from_parts(opcode.param1, ints[0]);
        let op2 = Operand::from_parts(opcode.param2, ints[1]);
        let dst = Destination::from_parts(opcode.param3, ints[2]);

        Self::new(dst, op1, op2).into()
    }
}

impl Instruction for EqualTo {
    fn execute<R: Iterator<Item = isize>, W: Sink<isize>>(
        &self,
        machine: &mut IntcodeMachine<R, W>,
    ) {
        let dst = self.dst.resolve(&machine);
        let op1 = self.op1.resolve(&machine);
        let op2 = self.op2.resolve(&machine);

        if op1 == op2 {
            debug!("EqualTo: memory[{}] = 1 ({} == {})", dst, op1, op2);
            machine.data[dst] = 1;
        } else {
            debug!("EqualTo: memory[{}] = 0 ({} != {})", dst, op1, op2);
            machine.data[dst] = 0;
        }
    }

    fn size(&self) -> usize {
        4
    }
}

#[derive(Debug)]
pub struct ModRelBase {
    operand: Operand,
}

impl ModRelBase {
    pub fn new(operand: Operand) -> Self {
        Self { operand }
    }

    fn decode(opcode: Opcode, ints: &[isize]) -> Instructions {
        let operand = Operand::from_parts(opcode.param1, ints[0]);

        Self::new(operand).into()
    }
}

impl Instruction for ModRelBase {
    fn execute<R: Iterator<Item = isize>, W: Sink<isize>>(
        &self,
        machine: &mut IntcodeMachine<R, W>,
    ) {
        let value = self.operand.resolve(&machine);

        debug!("ModRelBase: relative_base = {} (value: {})", machine.relative_base + value, value);

        machine.relative_base += value;
    }

    fn size(&self) -> usize {
        2
    }
}

#[enum_dispatch(Instructions)]
pub trait Instruction {
    fn execute<R: Iterator<Item = isize>, W: Sink<isize>>(
        &self,
        machine: &mut IntcodeMachine<R, W>,
    );

    fn size(&self) -> usize;
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
    ModRelBase,
    Halt,
}

impl Instructions {
    pub fn decode(ints: &[isize], #[cfg(debug_assertions)] ip: usize) -> Self {
        const ADD_OP: usize = 1;
        const MUL_OP: usize = 2;
        const INP_OP: usize = 3;
        const OUT_OP: usize = 4;
        const JIT_OP: usize = 5;
        const JIF_OP: usize = 6;
        const LST_OP: usize = 7;
        const EQU_OP: usize = 8;
        const MRB_OP: usize = 9;
        const HALT_OP: usize = 99;

        let opcode = Opcode::from(ints[0]);

        let bytes = &ints[1..];

        match opcode.opcode {
            ADD_OP => Add::decode(opcode, bytes),
            MUL_OP => Mul::decode(opcode, bytes),
            INP_OP => Input::decode(opcode, bytes),
            OUT_OP => Output::decode(opcode, bytes),
            JIT_OP => JumpIfTrue::decode(opcode, bytes),
            JIF_OP => JumpIfFalse::decode(opcode, bytes),
            LST_OP => LessThan::decode(opcode, bytes),
            EQU_OP => EqualTo::decode(opcode, bytes),
            MRB_OP => ModRelBase::decode(opcode, bytes),
            HALT_OP => Halt::new().into(),
            #[cfg(debug_assertions)]
            #[cold]
            n => panic!("Invalid opcode: {}, ip: {}", n, ip),
            #[cfg(not(debug_assertions))]
            #[cold]
            n => panic!("Invalid opcode: {}", n),
        }
    }
}

enum Mode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

impl From<usize> for Mode {
    fn from(i: usize) -> Self {
        match i {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => panic!("Invalid mode"),
        }
    }
}

struct Opcode {
    opcode: usize,
    param1: Mode,
    param2: Mode,
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

        Self { opcode, param1, param2, param3 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::once;

    fn parse_input(input: &str) -> Vec<isize> {
        input.split(',').map(str::parse).collect::<Result<_, _>>().unwrap()
    }

    #[test]
    fn conditionals() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let input = parse_input(input);
        let mut output = 0isize;
        let mut machine = IntcodeMachine::new(&input, once(7), &mut output);
        machine.run();
        assert_eq!(output, 999);

        let mut output = 0isize;
        let mut machine = IntcodeMachine::new(&input, once(8), &mut output);
        machine.run();
        assert_eq!(output, 1000);

        let mut output = 0isize;
        let mut machine = IntcodeMachine::new(&input, once(9), &mut output);
        machine.run();
        assert_eq!(output, 1001);
    }

    #[test]
    fn day_9_new_stuff() {
        let quine = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let inp = parse_input(quine);
        let mut output = Vec::new();
        let mut machine = IntcodeMachine::new(&inp, None.into_iter(), &mut output);
        machine.run();
        assert_eq!(inp, output);

        let input = "104,1125899906842624,99";
        let input = parse_input(input);
        let mut output = 0isize;
        let mut machine = IntcodeMachine::new(&input, None.into_iter(), &mut output);
        machine.run();
        assert_eq!(output, 1_125_899_906_842_624);

        let input = "1102,34915192,34915192,7,4,7,99,0";
        let input = parse_input(input);
        let mut output = 0isize;
        let mut machine = IntcodeMachine::new(&input, None.into_iter(), &mut output);
        machine.run();
        assert_eq!(output.to_string().len(), 16);
    }
}
