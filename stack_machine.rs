// # https://en.wikipedia.org/wiki/Stack_machine

const SIZE: usize = 2;

#[repr(usize)]
enum Op {
    Push(i32),
    Pop,
    Add,
    Sub,
    Mul,
}

#[derive(Debug)]
enum MachineErr {
    StackOverflow,
    StackUnderflow,
    IlligalInstruction,
}

#[derive(Debug)]
struct Machine<const N: usize> {
    stack: [i32; N], //Stack to hold instructions
    ip: usize,       // Instruction Pointer
}

impl<const T: usize> Machine<T> {
    fn new() -> Self {
        Self {
            stack: [0; T],
            ip: 0,
        }
    }

    fn step(&mut self, op: Op) -> Result<(), MachineErr> {
        match op {
            Op::Push(v) => {
                if self.ip >= SIZE {
                    return Err(MachineErr::StackOverflow);
                }
                self.stack[self.ip] = v;
                self.ip += 1;
                Ok(())
            }

            Op::Pop => {
                if self.ip == 0 {
                    return Err(MachineErr::StackUnderflow);
                }

                let _v = self.stack[self.ip - 1];
                self.ip -= 1;
                Ok(())
            }

            Op::Add => {
                if self.ip < 2 {
                    return Err(MachineErr::StackUnderflow);
                }

                let a = self.stack[self.ip - 1];
                self.ip -= 1;
                let b = self.stack[self.ip - 1];
                self.ip -= 1;

                self.step(Op::Push(a + b))
            }

            Op::Sub => {
                if self.ip < 2 {
                    return Err(MachineErr::StackUnderflow);
                }

                let a = self.stack[self.ip - 1];
                self.ip -= 1;
                let b = self.stack[self.ip - 1];
                self.ip -= 1;

                self.step(Op::Push(a - b))
            }

            Op::Mul => {
                if self.ip < 2 {
                    return Err(MachineErr::StackUnderflow);
                }

                let a = self.stack[self.ip - 1];
                self.ip -= 1;
                let b = self.stack[self.ip - 1];
                self.ip -= 1;

                self.step(Op::Push(a * b))
            }

            _ => Err(MachineErr::IlligalInstruction),
        }
    }

    fn dump(&self) {
        print!("STACK: [");
        for i in 0..self.ip {
            print!("{}, ", self.stack[i]);
        }
        print!("]");

        println!("");
    }
}

fn main() {
    let mut m = Machine::<SIZE>::new();

    for ins in [
        Op::Push(1),
        Op::Push(2),
        Op::Push(3),
        Op::Sub,
        Op::Mul,
        Op::Push(4),
        Op::Push(5),
        Op::Add,
        Op::Add,
    ] {
        match m.step(ins) {
            Ok(()) => m.dump(),
            Err(e) => {
                eprintln!("Error: {:?}", e);
                break;
            }
        }
    }
}
