use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, Clone)]
pub enum Value {
    Old,
    Num(u8),
}

impl FromStr for Value {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Value::Old),
            x => Ok(Value::Num(x.parse()?)),
        }
    }
}

// Left value of operation is always `Old`
#[derive(Debug, Clone)]
pub enum Operation {
    Mul(Value),
    Add(Value),
}

#[derive(Debug, Clone)]
pub struct Monkey {
    pub items: Vec<u64>,
    pub op: Operation,
    pub test_divisor: u8,
    pub test_true_target: usize,
    pub test_false_target: usize,
}
