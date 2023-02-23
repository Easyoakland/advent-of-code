use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
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
#[derive(Debug)]
pub enum Operation {
    Mul(Value),
    Add(Value),
}

#[derive(Debug)]
pub struct Monkey {
    pub starting_items: Vec<u8>,
    pub op: Operation,
    pub test_divisor: u8,
    pub test_true_target: usize,
    pub test_false_target: usize,
}
