// # https://en.wikipedia.org/wiki/Stack_machine

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

#[derive(Debug)]
enum MachineErr {
    StackOverflow,
    StackUnderflow,
    IlligalInstruction,
}

#[derive(Debug)]
struct Machine<const N: usize> {
    stack: [i32; N], // Stack to hold instructions
    sp: usize,

    program: Vec<Op>, //Program stack as list of instructions
    ip: usize,        // Instruction Pointer
}

impl<const T: usize> Machine<T> {
    fn new(program: Vec<Op>) -> Self {
        Self {
            stack: [0; T],
            sp: 0,

            program,
            ip: 0,
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

            _ => Err(MachineErr::IlligalInstruction),
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

fn main() {
    let _program = vec![
        Op::Push(1),
        Op::Push(2),
        Op::Push(3),
        Op::Dup(3),
        Op::Sub,
        Op::Mul,
        Op::Push(4),
        Op::Push(5),
        Op::Add,
        Op::Add,
        Op::Halt,
    ];

    let program = vec![
        Op::Push(0),
        Op::Push(1),
        Op::Dup(1),
        Op::Dup(1),
        Op::Add,
        Op::Jump(2),
        Op::Halt,
    ];

    let mut m = Machine::<SIZE>::new(program);

    for _ in 0..PROG_SIZE {
        match m.step() {
            Ok(()) => m.dump(),
            Err(e) => {
                eprintln!("Error: {:?}", e);
                break;
            }
        }
    }
}
