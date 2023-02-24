mod data;
mod parse;
use crate::data::{Monkey, Operation, Value};
use crate::parse::{parse_final, Span};
use std::{error::Error, fs};
fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("Part 1 answer: {:#?}", part1::run("input.txt")?);
    println!("Part 2 answer: {:#?}", part2::run("input.txt")?);
    Ok(())
}

mod part1 {
    use super::*;
    fn do_round(monkeys: &mut Vec<Monkey>, inspections: &mut [usize]) {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();
            let op = monkey.op;
            let test_divisor = monkey.test_divisor;
            let test_true = monkey.test_true_target;
            let test_false = monkey.test_false_target;

            // Monkey will always inspect all its items.
            inspections[i] += monkeys[i].items.len();
            // let mut j = 0;
            let j = 0; // Index doesn't move (look at end of loop for why).
            while j < monkeys[i].items.len() {
                // Items edited on the actual monkey not the clone.
                let item = &mut monkeys[i].items[j];
                // Change worry level accordingly
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
                // After inspection worrylevel divide by 3.
                *item /= 3;
                // Check and handle worry level matching
                if *item % u64::from(test_divisor) == 0 {
                    let moved_item = monkeys[i].items.swap_remove(j);
                    monkeys[test_true].items.push(moved_item);
                } else {
                    let moved_item = monkeys[i].items.swap_remove(j);
                    monkeys[test_false].items.push(moved_item);
                }
                // Below cancel each other.
                // j -= 1; // remove 1 for item that removed.
                // j += 1; // increment loop
            }
        }
    }

    pub fn run(file: &str) -> Result<u32, Box<dyn Error + Send + Sync>> {
        let input_str = fs::read_to_string(file)?;
        let input = Span::new(Box::leak(Box::new(input_str)));
        let mut monkeys = parse_final(input)?;
        let mut inspections = vec![0; monkeys.len()];
        for _ in 0..20 {
            do_round(&mut monkeys, &mut inspections);
        }
        let max = *inspections.iter().max().unwrap();
        let inspections: Vec<_> = inspections.into_iter().filter(|&x| x != max).collect();
        let max2 = inspections.iter().max().unwrap();

        Ok((max * max2).try_into().unwrap())
    }
}

mod part2 {
    use super::*;
    fn do_round(monkeys: &mut Vec<Monkey>, inspections: &mut [usize]) {
        let mut test_divisors = Vec::with_capacity(monkeys.len());
        for monkey in monkeys.iter() {
            test_divisors.push(u64::from(monkey.test_divisor));
        }
        let common_test_divisor: u64 = test_divisors.iter().product();

        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();
            let op = monkey.op;
            let test_divisor = monkey.test_divisor;
            let test_true = monkey.test_true_target;
            let test_false = monkey.test_false_target;

            // Monkey will always inspect all its items.
            inspections[i] += monkeys[i].items.len();
            // let mut j = 0;
            let j = 0; // Index doesn't move (look at end of loop for why).
            while j < monkeys[i].items.len() {
                // Items edited on the actual monkey not the clone.
                let item = &mut monkeys[i].items[j];
                // Change worry level accordingly
                // Note that all divisible values in input are prime (not mentioned in test but apparently the case).
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
                // Remove multiples of common divisor because they are superfluous.
                // Only cares about offset from a multiple for matching.
                *item %= common_test_divisor;
                // Check and handle worry level matching
                if *item % (u64::from(test_divisor)) == u64::from(0u8) {
                    let moved_item = monkeys[i].items.swap_remove(j);
                    monkeys[test_true].items.push(moved_item);
                } else {
                    let moved_item = monkeys[i].items.swap_remove(j);
                    monkeys[test_false].items.push(moved_item);
                }
                // Below cancel each other.
                // j -= 1; // remove 1 for item that removed.
                // j += 1; // increment loop
            }
        }
    }

    pub fn run(file: &str) -> Result<u64, Box<dyn Error + Send + Sync>> {
        let input_str = fs::read_to_string(file)?;
        let input = Span::new(Box::leak(Box::new(input_str)));
        let mut monkeys = parse_final(input)?;
        let mut inspections = vec![0; monkeys.len()];
        for i in 0..10000 {
            do_round(&mut monkeys, &mut inspections);
            // DEBUG. Below was used because actually doing multiplications results in a number so big the limiter on speed is multiplication (using bigint). Was trying to debug using below to see what took so long.
            if i % 1000 == 0 {
                println!("{i}");
                //     dbg!(&monkeys[0].items[0]);
            }
        }
        let max = *inspections.iter().max().unwrap();
        let inspections: Vec<_> = inspections.into_iter().filter(|&x| x != max).collect();
        let max2 = inspections.iter().max().unwrap();

        Ok((max * max2).try_into().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error + Send + Sync>> {
        assert_eq!(part1::run("inputtest.txt")?, 10605);
        Ok(())
    }

    #[test]
    fn test_part1_ans() -> Result<(), Box<dyn Error + Send + Sync>> {
        assert_eq!(part1::run("input.txt")?, 66124);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error + Send + Sync>> {
        assert_eq!(part2::run("inputtest.txt")?, 2713310158);
        Ok(())
    }

    #[test]
    fn test_part2_ans() -> Result<(), Box<dyn Error + Send + Sync>> {
        assert_eq!(part2::run("input.txt")?, 19309892877);
        Ok(())
    }
}
