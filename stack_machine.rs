//  https://en.wikipedia.org/wiki/Stack_machine

use std::fs;
use std::str::FromStr;

const SIZE: usize = 24;
const PROG_SIZE: usize = 25;

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum Op {
    NoOp,
    Push(i32),
    Pop,
    Add,
    Sub,
    Mul,
    Jump(usize),
    Dup(usize),
    Halt,
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ops = s.trim().split(' ').collect::<Vec<&str>>();

        let op = match ops[0] {
            "push" if ops.len() == 2 => Ok(Op::Push(
                ops[1].parse::<i32>().expect("Error: when parsing push"),
            )),

            "jump" if ops.len() == 2 => Ok(Op::Jump(
                ops[1].parse::<usize>().expect("Error: when parsing jump"),
            )),

            "dup" if ops.len() == 2 => Ok(Op::Dup(
                ops[1].parse::<usize>().expect("Error: when parsing dup"),
            )),

            "pop" => Ok(Op::Pop),
            "add" => Ok(Op::Add),
            "sub" => Ok(Op::Sub),
            "mul" => Ok(Op::Mul),
            "halt" => Ok(Op::Halt),
            "noop" => Ok(Op::NoOp),

            _ => Err(format!("Error: Unable to parse {s:?}")),
        };

        op
    }
}

#[derive(Debug)]
enum MachineErr {
    StackOverflow,
    StackUnderflow,
}

#[derive(Debug)]
struct Machine<const N: usize> {
    stack: [i32; N], // Stack to hold instructions
    sp: usize,

    program: Vec<Op>, //Program stack as list of instructions
    ip: usize,        // Instruction Pointer

    halt: bool,
}

impl<const T: usize> Machine<T> {
    fn new(program: Vec<Op>) -> Self {
        Self {
            stack: [0; T],
            sp: 0,

            program,
            ip: 0,
            halt: false,
        }
    }

    fn step(&mut self) -> Result<(), MachineErr> {
        let op = self.program[self.ip];
        match op {
            Op::Push(v) => {
                if self.sp >= SIZE {
                    return Err(MachineErr::StackOverflow);
                }
                self.stack[self.sp] = v;
                self.sp += 1;
                self.ip += 1;

                Ok(())
            }

            Op::Pop => {
                if self.sp == 0 {
                    return Err(MachineErr::StackUnderflow);
                }

                let _v = self.stack[self.sp - 1];
                self.sp -= 1;
                self.ip += 1;

                Ok(())
            }

            Op::Add => {
                if self.sp < 2 {
                    return Err(MachineErr::StackUnderflow);
                }

                let a = self.stack[self.sp - 1];
                self.sp -= 1;
                let b = self.stack[self.sp - 1];
                self.sp -= 1;

                self.stack[self.sp] = a + b;
                self.sp += 1;
                self.ip += 1;

                Ok(())
            }

            Op::Sub => {
                if self.sp < 2 {
                    return Err(MachineErr::StackUnderflow);
                }

                let a = self.stack[self.sp - 1];
                self.sp -= 1;
                let b = self.stack[self.sp - 1];
                self.sp -= 1;

                self.stack[self.sp] = a - b;
                self.sp += 1;
                self.ip += 1;

                Ok(())
            }

            Op::Mul => {
                if self.sp < 2 {
                    return Err(MachineErr::StackUnderflow);
                }

                let a = self.stack[self.sp - 1];
                self.sp -= 1;
                let b = self.stack[self.sp - 1];
                self.sp -= 1;

                self.stack[self.sp] = a * b;
                self.sp += 1;
                self.ip += 1;

                Ok(())
            }

            Op::Jump(v) => {
                self.ip = v;

                Ok(())
            }

            Op::Dup(v) => {
                self.stack[self.sp] = self.stack[self.sp - 1 - v];

                self.sp += 1;
                self.ip += 1;

                Ok(())
            }

            Op::Halt => {
                self.halt = true;
                self.ip += 1;
                Ok(())
            }

            Op::NoOp => {
                self.ip += 1;
                Ok(())
            }
        }
    }

    fn dump(&self) {
        print!("STACK: [");
        for i in 0..self.sp {
            print!("{}, ", self.stack[i]);
        }
        print!("]");

        println!("");
    }
}

fn read_source_file(sf: &str) -> Vec<Op> {
    let f = fs::read_to_string(sf).expect("Error: Unable to read file {sf:?}");

    let t = f
        .trim()
        .split('\n')
        .map(|x| Op::from_str(x).unwrap())
        .collect::<Vec<Op>>();

    t
}

fn main() {
    let mut e = std::env::args().into_iter();

    let file_name = e.nth(1);

    if file_name.is_none() {
        eprintln!("USAGE: ./stack_machine *.vm");
        return;
    }

    let prog = read_source_file(&file_name.unwrap());

    let mut m = Machine::<SIZE>::new(prog);

    for _ in 0..PROG_SIZE {
        if m.halt {
            break;
        }

        match m.step() {
            Ok(()) => m.dump(),
            Err(e) => {
                eprintln!("Error: {:?}", e);
                break;
            }
        }
    }
}
