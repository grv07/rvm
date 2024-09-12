//  https://en.wikipedia.org/wiki/Stack_machine
mod ins;
mod word;

use ins::Ins;
use std::fs;
use std::io::Write;
use std::ops::{Add, Div, Mul, Sub};
use word::Word;

const SIZE: usize = 24;

#[derive(Debug)]
enum MachineErr {
    StackOverflow,
    StackUnderflow,
}

#[derive(Debug)]
struct Machine<const N: usize> {
    stack: [Word; N], // Stack to hold instructions
    sp: usize,

    program: Vec<Ins>, //Program stack as list of instructions
    ip: usize,         // Instruction Pointer

    halt: bool,
}

impl<const T: usize> Machine<T> {
    fn new(program: Vec<Ins>) -> Self {
        Self {
            stack: [Word::Int(0); T],
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
        let ins = self.program[self.ip];
        match ins {
            Ins::Push(v) => {
                if self.sp >= SIZE {
                    return Err(MachineErr::StackOverflow);
                }
                self.stack[self.sp] = v;
                // self.stack.push(v);
                // = v;
                self.sp += 1;
                self.ip += 1;

                Ok(())
            }

            Ins::Pop => {
                if self.sp == 0 {
                    return Err(MachineErr::StackUnderflow);
                }

                let _v = self.stack[self.sp - 1];
                self.sp -= 1;
                self.ip += 1;

                Ok(())
            }

            Ins::Swap(v) => {
                if self.sp < v {
                    return Err(MachineErr::StackUnderflow);
                }

                let a = self.sp - 1;
                let b = self.sp - 1 - v;

                let t = self.stack[a];
                self.stack[a] = self.stack[b];
                self.stack[b] = t;

                self.ip += 1;

                Ok(())
            }

            Ins::Not => {
                // self.stack[self.sp - 1] = Word::Boolean(self.stack[self.sp - 1]);

                self.ip += 1;

                Ok(())
            }

            Ins::Gef => {
                if self.sp < 2 {
                    return Err(MachineErr::StackUnderflow);
                }

                self.stack[self.sp - 2] =
                    Word::Boolean(self.stack[self.sp - 1] >= self.stack[self.sp - 2]);

                self.sp -= 1;
                self.ip += 1;

                Ok(())
            }

            Ins::AddI => {
                if self.sp < 2 {
                    return Err(MachineErr::StackUnderflow);
                }

                let a = self.stack[self.sp - 1];
                self.sp -= 1;
                let b = self.stack[self.sp - 1];
                self.sp -= 1;

                self.stack[self.sp] = a.add(b).unwrap();
                self.sp += 1;
                self.ip += 1;

                Ok(())
            }

            Ins::SubI => {
                if self.sp < 2 {
                    return Err(MachineErr::StackUnderflow);
                }

                let a = self.stack[self.sp - 1];
                self.sp -= 1;
                let b = self.stack[self.sp - 1];
                self.sp -= 1;

                self.stack[self.sp] = b.sub(a).unwrap();
                self.sp += 1;
                self.ip += 1;

                Ok(())
            }

            Ins::MulI => {
                if self.sp < 2 {
                    return Err(MachineErr::StackUnderflow);
                }

                let a = self.stack[self.sp - 1];
                self.sp -= 1;
                let b = self.stack[self.sp - 1];
                self.sp -= 1;

                self.stack[self.sp] = b.mul(a).unwrap();
                self.sp += 1;
                self.ip += 1;

                Ok(())
            }

            Ins::DivI => {
                if self.sp < 2 {
                    return Err(MachineErr::StackUnderflow);
                }

                let a = self.stack[self.sp - 1];
                self.sp -= 1;
                let b = self.stack[self.sp - 1];
                self.sp -= 1;

                self.stack[self.sp] = b.div(a).unwrap();
                self.sp += 1;
                self.ip += 1;

                Ok(())
            }

            Ins::AddF => {
                if self.sp < 2 {
                    return Err(MachineErr::StackUnderflow);
                }

                let a = self.stack[self.sp - 1];
                self.sp -= 1;
                let b = self.stack[self.sp - 1];
                self.sp -= 1;

                self.stack[self.sp] = (a + b).expect("Unable to add");
                self.sp += 1;
                self.ip += 1;

                Ok(())
            }

            Ins::SubF => {
                if self.sp < 2 {
                    return Err(MachineErr::StackUnderflow);
                }

                let a = self.stack[self.sp - 1];
                self.sp -= 1;
                let b = self.stack[self.sp - 1];
                self.sp -= 1;

                self.stack[self.sp] = b.sub(a).unwrap();
                self.sp += 1;
                self.ip += 1;

                Ok(())
            }

            Ins::MulF => {
                if self.sp < 2 {
                    return Err(MachineErr::StackUnderflow);
                }

                let a = self.stack[self.sp - 1];
                self.sp -= 1;
                let b = self.stack[self.sp - 1];
                self.sp -= 1;

                self.stack[self.sp] = b.mul(a).unwrap();
                self.sp += 1;
                self.ip += 1;

                Ok(())
            }

            Ins::DivF => {
                if self.sp < 2 {
                    return Err(MachineErr::StackUnderflow);
                }

                let a = self.stack[self.sp - 1];
                self.sp -= 1;
                let b = self.stack[self.sp - 1];
                self.sp -= 1;

                self.stack[self.sp] = b.div(a).unwrap();
                self.sp += 1;
                self.ip += 1;

                Ok(())
            }

            Ins::Jump(v) => {
                self.ip = v;

                Ok(())
            }

            Ins::JumpIf(v) => {
                if self.sp < 1 {
                    return Err(MachineErr::StackUnderflow);
                }

                if self.stack[self.sp - 1].is_true() {
                    self.ip = v;
                } else {
                    self.ip += 1;
                }

                self.sp -= 1;

                Ok(())
            }

            Ins::Dup(v) => {
                self.stack[self.sp] = self.stack[self.sp - 1 - v];

                self.sp += 1;
                self.ip += 1;

                Ok(())
            }

            Ins::Halt => {
                self.halt = true;
                self.ip += 1;
                Ok(())
            }

            Ins::NoOp => {
                self.ip += 1;
                Ok(())
            }
        }
    }

    fn dump(&self) {
        let ins = self.program[self.ip - 1];
        println!("{ins:?}");
        print!("STACK: [");
        for i in 0..self.sp {
            print!("{:?}, ", self.stack[i]);
        }
        print!("]");

        println!("");
    }
}

fn read_source_file(sf: &str) -> Vec<Ins> {
    let vm_file = fs::read_to_string(sf).expect("Error: Unable to read file {sf:?}");

    let ops = vm_file
        .trim()
        .split('\n')
        .filter(|x| !x.starts_with('#'))
        .filter(|x| !x.is_empty());

    let lable_table = ops
        .clone()
        .enumerate()
        .map(|(i, v)| (v, i))
        .filter(|(v, _)| v.ends_with(':'))
        .map(|(v, i)| (v.replace(':', ""), i))
        .collect::<std::collections::HashMap<_, _>>();

    let ops = ops
        .filter(|v| !v.ends_with(':'))
        .map(|x| Ins::to_ins(x, &lable_table).unwrap())
        .collect::<Vec<Ins>>();

    ops
}

fn main() {
    let mut args = std::env::args().into_iter();

    let mut file_name = String::new();
    let mut limit = -1;
    let mut debug = false;

    for arg in args {
        if arg.ends_with(".vm") {
            file_name = arg.clone();
        }

        if arg.starts_with("-l=") {
            eprintln!("USAGE: -l=limit");
            limit = arg.replace("-l=", "").parse::<i32>().unwrap();
        }

        if arg == "-d" {
            eprintln!("USAGE: debug, -d");
            debug = true;
        }
    }

    if file_name.len() < 3 || limit == -1 {
        eprintln!("USAGE: ./stack_machine *.vm");
        eprintln!("USAGE: -l=limit");
        eprintln!("USAGE: debug,  -d");
        eprintln!("ERROR: Expect a input");

        return;
    }

    let prog = read_source_file(&file_name);

    let mut m = Machine::<SIZE>::new(prog);

    for _ in 0..limit {
        if m.halt {
            break;
        }

        match m.step() {
            Ok(()) => {
                if debug {
                    m.dump();
                }
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                break;
            }
        }
    }

    let _ = m.save_prog_to_file("game.bin");
}
