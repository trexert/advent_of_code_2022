use num::Num;
use std::{error::Error, fmt::Display, str::FromStr};

#[derive(Clone, Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    pub fn call<T: Num>(&self, left: T, right: T) -> T {
        match self {
            Self::Add => left + right,
            Self::Sub => left - right,
            Self::Mul => left * right,
            Self::Div => left / right,
        }
    }
}

impl FromStr for Op {
    type Err = ParseOpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Sub),
            "*" => Ok(Self::Mul),
            "/" => Ok(Self::Div),
            _ => Err(ParseOpError {}),
        }
    }
}

impl ToString for Op {
    fn to_string(&self) -> String {
        match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
        }
        .to_owned()
    }
}

#[derive(Debug)]
pub struct ParseOpError {}

impl Error for ParseOpError {}

impl Display for ParseOpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Input not +, -, *, or /")
    }
}
