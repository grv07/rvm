use std::collections::HashMap;
use std::convert::TryFrom;
use word::Word;

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Ins {
    NoOp,
    Push(Word),
    Pop,

    AddI,
    SubI,
    MulI,
    DivI,

    AddF,
    SubF,
    MulF,
    DivF,

    Gef,
    Not,

    Jump(usize),
    JumpIf(usize),
    Dup(usize),
    Swap(usize),

    Halt,
}

impl ToString for Ins {
    fn to_string(&self) -> String {
        match self {
            Ins::Push(v) => format!("push {}\n", v),
            Ins::Jump(v) => format!("jump {}\n", v),
            Ins::JumpIf(v) => format!("jumpif {}\n", v),
            Ins::Dup(v) => format!("dup {}\n", v),
            Ins::Swap(v) => format!("swap {}\n", v),

            Ins::Gef => String::from("gef\n"),
            Ins::Not => String::from("not\n"),

            Ins::AddI => String::from("addi\n"),
            Ins::SubI => String::from("subi\n"),
            Ins::MulI => String::from("muli\n"),
            Ins::DivI => String::from("divi\n"),

            Ins::AddF => String::from("addf\n"),
            Ins::SubF => String::from("subf\n"),
            Ins::MulF => String::from("mulf\n"),
            Ins::DivF => String::from("divf\n"),

            Ins::Pop => String::from("pop\n"),

            Ins::Halt => String::from("halt\n"),
            Ins::NoOp => String::from("noop\n"),
        }
    }
}

impl Ins {
    pub fn to_ins(line: &str, lt: &HashMap<String, usize>) -> Result<Self, String> {
        let line = if line.contains('#') {
            line.split('#').nth(0).expect("Unable to parse")
        } else {
            line
        };

        let ops = line.trim().split(&[' ']).collect::<Vec<&str>>();

        println!(" == {:?}", ops);

        let op = match ops[0] {
            "push" if ops.len() == 2 => {
                let word: Word = Word::try_from(ops[1]).unwrap();
                Ok(Ins::Push(word))
            }

            "swap" if ops.len() == 2 => Ok(Ins::Swap(
                ops[1].parse::<usize>().expect("Error: when parsing swap"),
            )),

            "jump" if ops.len() == 2 => match ops[1].parse::<usize>() {
                Ok(v) => Ok(Ins::Jump(v)),
                Err(_) => {
                    let v = ops[1];
                    if let Some(v) = lt.get(v) {
                        Ok(Ins::Jump(*v))
                    } else {
                        Err(format!("Error: Unable to parse label/index for jump {v}"))
                    }
                }
            },

            "jumpif" if ops.len() == 2 => match ops[1].parse::<usize>() {
                Ok(v) => Ok(Ins::Jump(v)),
                Err(_) => {
                    let v = ops[1];
                    if let Some(v) = lt.get(v) {
                        Ok(Ins::JumpIf(*v))
                    } else {
                        Err(format!("Error: Unable to parse label/index for jumpif {v}"))
                    }
                }
            },

            "dup" if ops.len() == 2 => Ok(Ins::Dup(
                ops[1].parse::<usize>().expect("Error: when parsing dup"),
            )),

            "pop" => Ok(Ins::Pop),
            "gef" => Ok(Ins::Gef),
            "not" => Ok(Ins::Not),

            "addi" => Ok(Ins::AddI),
            "subi" => Ok(Ins::SubI),
            "muli" => Ok(Ins::MulI),
            "divi" => Ok(Ins::DivI),

            "addf" => Ok(Ins::AddF),
            "subf" => Ok(Ins::SubF),
            "mulf" => Ok(Ins::MulF),
            "divf" => Ok(Ins::DivF),

            "halt" => Ok(Ins::Halt),
            "noop" => Ok(Ins::NoOp),

            _ => Err(format!("Error: Unable to parse > {line}")),
        };

        op
    }
}
