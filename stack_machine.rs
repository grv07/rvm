// # https://en.wikipedia.org/wiki/Stack_machine

#[repr(usize)]
enum Op {
    NoOp,
    Push(i32),
    Pop,
    Add,
    Sub,
    Mul,
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

    fn step(&mut self, op: Op) -> Option<i32> {
        match op {
            Op::NoOp => {
                println!("No operation");
                None
            }
            Op::Push(v) => {
                self.stack[self.ip] = v;
                self.ip += 1;
                None
            }
            Op::Pop => {
                let v = self.stack[self.ip - 1];
                self.ip -= 1;
                return Some(v);
            }
            Op::Add => {
                if let (Some(a), Some(b)) = (self.step(Op::Pop), self.step(Op::Pop)) {
                    let res = a + b;
                    self.step(Op::Push(res))
                } else {
                    eprintln!("Add is not a valid operation on current state of stack ");
                    None
                }
            }
            Op::Sub => {
                if let (Some(a), Some(b)) = (self.step(Op::Pop), self.step(Op::Pop)) {
                    let res = a - b;
                    self.step(Op::Push(res))
                } else {
                    eprintln!("Add is not a valid operation on current state of stack ");
                    None
                }
            }
            Op::Mul => {
                if let (Some(a), Some(b)) = (self.step(Op::Pop), self.step(Op::Pop)) {
                    let res = a * b;
                    self.step(Op::Push(res))
                } else {
                    eprintln!("Add is not a valid operation on current state of stack ");
                    None
                }
            }
        }
    }

    fn dump(&self) {
        for i in &self.stack {
            print!("{i}, ");
        }
        print!("]");

        println!("");
    }
}

fn main() {
    const SIZE: usize = 16;
    let mut m = Machine::<SIZE>::new();

    for ins in [
        Op::Push(1),
        Op::Push(2),
        Op::Push(3),
        Op::Sub,
        Op::NoOp,
        Op::Mul,
        Op::Push(4),
        Op::Push(5),
        Op::Add,
        Op::Add,
    ] {
        m.step(ins);
        m.dump();
    }
}
