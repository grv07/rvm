use std::convert::TryFrom;
use std::fmt;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Word {
    Float(f64),
    Boolean(bool),
    Int(i64),
    Usize(usize),
}

impl Word {
    pub fn is_true(&self) -> bool {
        match self {
            Word::Float(x) => *x > 0.0,
            Word::Int(x) => *x > 0,
            Word::Usize(x) => *x > 0,
            Word::Boolean(x) => *x,
        }
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Word::Float(x) => write!(f, "{}", x),
            Word::Int(x) => write!(f, "{}", x),
            Word::Usize(x) => write!(f, "{}", x),
            Word::Boolean(x) => write!(f, "{}", x),
        }
    }
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
            (Word::Float(a), Word::Float(b)) => Ok(Word::Float(round_to_ten_digits(a * b))),
            _ => Err(format!(
                "Error: Operation Mul not supported yet for {:?} {:?}",
                self, other
            )),
        }
    }
}

impl Div for Word {
    type Output = Result<Self, String>;

    fn div(self, other: Word) -> Result<Self, String> {
        match (self, other) {
            (Word::Int(b), Word::Int(a)) => Ok(Word::Int(b / a)),
            (Word::Float(b), Word::Float(a)) => Ok(Word::Float(round_to_ten_digits(b / a))),
            _ => Err(format!(
                "Error: Operation Div not supported yet for {:?} {:?}",
                self, other
            )),
        }
    }
}

fn round_to_ten_digits(num: f64) -> f64 {
    // println!("{num}");
    let factor = 10f64.powi(10);
    (num * factor).round() / factor
}
