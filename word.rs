use std::convert::TryFrom;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Word {
    Float(f64),
    Int(i64),
    Usize(usize),
}

impl TryFrom<&str> for Word {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.contains('.') {
            Ok(Word::Float(s.parse::<f64>().unwrap()))
        } else {
            Ok(Word::Int(s.parse::<i64>().unwrap()))
        }
    }
}

impl Add for Word {
    type Output = Result<Self, String>;

    fn add(self, other: Word) -> Result<Self, String> {
        match (self, other) {
            (Word::Int(a), Word::Int(b)) => Ok(Word::Int(a + b)),
            (Word::Float(a), Word::Float(b)) => Ok(Word::Float(a + b)),
            _ => Err(format!(
                "Error: Operation Add not supported yet for {:?} {:?}",
                self, other
            )),
        }
    }
}

impl Sub for Word {
    type Output = Result<Self, String>;

    fn sub(self, other: Word) -> Result<Self, String> {
        match (self, other) {
            (Word::Int(a), Word::Int(b)) => Ok(Word::Int(a - b)),
            (Word::Float(a), Word::Float(b)) => Ok(Word::Float(a - b)),
            _ => Err(format!(
                "Error: Operation Sub not supported yet for {:?} {:?}",
                self, other
            )),
        }
    }
}

impl Mul for Word {
    type Output = Result<Self, String>;

    fn mul(self, other: Word) -> Result<Self, String> {
        match (self, other) {
            (Word::Int(a), Word::Int(b)) => Ok(Word::Int(a * b)),
            (Word::Float(a), Word::Float(b)) => Ok(Word::Float(a * b)),
            _ => Err(format!(
                "Error: Operation Mul not supported yet for {:?} {:?}",
                self, other
            )),
        }
    }
}
