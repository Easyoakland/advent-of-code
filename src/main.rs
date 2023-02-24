use std::{error::Error, fs};
mod data;
mod parse;
use crate::data::Monkey;
use crate::parse::{parse_final, Span};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("Part 1 answer: {:#?}", part1::run("input.txt")?);
    Ok(())
}

mod part1 {
    use crate::data::{Operation, Value};

    use super::*;
    fn do_round(monkeys: &mut Vec<Monkey>) {
        for monkey in monkeys.iter_mut() {
            let op = monkey.op.clone();
            for item in monkey.items.iter_mut() {
                match op {
                    Operation::Mul(ref value) => match value {
                        Value::Old => *item *= *item,
                        Value::Num(x) => *item *= u64::from(*x),
                    },
                    Operation::Add(ref value) => match value {
                        Value::Old => *item += *item,
                        Value::Num(x) => *item += u64::from(*x),
                    },
                }
            }
        }
    }

    pub fn run(file: &str) -> Result<u32, Box<dyn Error + Send + Sync>> {
        let input_str = fs::read_to_string(file)?;
        let input = Span::new(Box::leak(Box::new(input_str)));
        let mut monkeys = parse_final(input)?;
        for _ in 1..20 {
            do_round(&mut monkeys);
            dbg!(&monkeys);
        }

        Ok(0)
    }
}
