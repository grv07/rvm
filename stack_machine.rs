//  https://en.wikipedia.org/wiki/Stack_machine

use std::fs;
use std::io::Write;
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

impl ToString for Op {
    fn to_string(&self) -> String {
        match self {
            Op::Push(v) => format!("push {}\n", v),
            Op::Jump(v) => format!("jump {}\n", v),
            Op::Dup(v) => format!("dup {}\n", v),

            Op::Add => String::from("add\n"),
            Op::Sub => String::from("sub\n"),
            Op::Mul => String::from("mul\n"),
            Op::Pop => String::from("pop\n"),

            Op::Halt => String::from("halt\n"),
            Op::NoOp => String::from("noop\n"),
        }
    }
}

impl FromStr for Op {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line = if line.contains('#') {
            line.split('#').nth(0).expect("Unable to parse")
        } else {
            line
        };

        let ops = line.trim().split(&[' ']).collect::<Vec<&str>>();

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

            _ => Err(format!("Error: Unable to parse {line}")),
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

    fn save_prog_to_file(&self, file: &str) -> Result<usize, std::io::Error> {
        let mut f = fs::File::create(file)?;

        let mut p = self.program.iter().map(|v| v.to_string());

        while let Some(v) = p.next() {
            let _ = f.write_all(v.as_bytes())?;
        }

        f.flush()?;

        Ok(0)
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
        .filter(|x| !x.starts_with('#'))
        .map(|x| Op::from_str(x).unwrap())
        .collect::<Vec<Op>>();

    t
}

fn main() {
    let mut e = std::env::args().into_iter();

    let file_name = e.nth(1);

    if file_name.is_none() {
        eprintln!("USAGE: ./stack_machine *.vm");
        eprintln!("ERROR: Expect a input");
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

    let _ = m.save_prog_to_file("game.bin");
}
