use std::error::Error;
use std::ops::{Add, Div, Mul, Sub};

mod data {
    use super::*;
    use std::collections::BTreeMap;

    pub type Val = usize;
    pub type MonkeyId<'a> = &'a str;
    pub type MonkeyOp<'b> =
        for<'a> fn(&'a Monkey<'b>, &'a Monkey<'b>, &'a MonkeyBacking<'b>) -> Val;
    pub type MonkeyBacking<'a> = BTreeMap<MonkeyId<'a>, Monkey<'a>>;

    pub fn monkey_op<'a, 'b>(
        a: &'a Monkey<'b>,
        b: &'a Monkey<'b>,
        op: fn(Val, Val) -> Val,
        backing: &'a MonkeyBacking<'b>,
    ) -> Val {
        match (a, b) {
            (Monkey::Const(a), Monkey::Const(b)) => op(*a, *b),
            (Monkey::Const(a), Monkey::Op(b)) => {
                op(*a, b.0(&backing[b.1], &backing[b.2], &backing))
            }
            (Monkey::Op(a), Monkey::Const(b)) => {
                op(a.0(&backing[a.1], &backing[a.2], &backing), *b)
            }
            (Monkey::Op(a), Monkey::Op(b)) => op(
                a.0(&backing[a.1], &backing[a.2], &backing),
                b.0(&backing[b.1], &backing[b.2], &backing),
            ),
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum Monkey<'a> {
        Const(Val),
        Op((MonkeyOp<'a>, MonkeyId<'a>, MonkeyId<'a>)),
    }

    impl<'a> Monkey<'a> {
        pub fn add<'b>(&'b self, rhs: &'b Self, backing: &'b MonkeyBacking<'a>) -> Val {
            monkey_op(self, rhs, Val::add, backing)
        }
        pub fn sub<'b>(&'b self, rhs: &'b Self, backing: &'b MonkeyBacking<'a>) -> Val {
            monkey_op(self, rhs, Val::sub, backing)
        }
        pub fn mul<'b>(&'b self, rhs: &'b Self, backing: &'b MonkeyBacking<'a>) -> Val {
            monkey_op(self, rhs, Val::mul, backing)
        }
        pub fn div<'b>(&'b self, rhs: &'b Self, backing: &'b MonkeyBacking<'a>) -> Val {
            monkey_op(self, rhs, Val::div, backing)
        }

        pub fn eval(&self, backing: &MonkeyBacking<'a>) -> Val {
            match self {
                Monkey::Const(x) => *x,
                Monkey::Op((op, a, b)) => op(&backing[a], &backing[b], &backing),
            }
        }
    }
}

mod parse {
    use crate::data::{Monkey, MonkeyId, MonkeyOp, Val};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, digit1, line_ending, one_of},
        combinator::{all_consuming, eof},
        multi::separated_list1,
        sequence::{preceded, terminated, tuple},
        IResult, Parser,
    };
    use std::collections::BTreeMap;

    pub fn monkey_op(input: &str) -> IResult<&str, (&str, MonkeyOp, &str)> {
        tuple((
            terminated(alpha1, tag(" ")),
            one_of("+-*/").map(|x| match x {
                '+' => Monkey::add,
                '-' => Monkey::sub,
                '*' => Monkey::mul,
                '/' => Monkey::div,
                _ => unreachable!(),
            }),
            preceded(tag(" "), alpha1),
        ))(input)
    }

    enum Op<'a> {
        Const(Val),
        Op((MonkeyOp<'a>, &'a str, &'a str)),
    }

    pub fn monkey<'a>(
        input: &'a str,
        all_monkeys: &mut BTreeMap<MonkeyId<'a>, Monkey<'a>>,
    ) -> IResult<&'a str, ()> {
        let (input, id) = terminated(alpha1, tag(": "))(input)?;
        let (input, op): (&str, Op) = alt((
            digit1.map(|x: &str| Op::Const(x.parse::<Val>().unwrap())),
            monkey_op.map(|x| Op::Op((x.1, x.0, x.2))),
        ))(input)?;
        match op {
            Op::Const(x) => all_monkeys.insert(id, Monkey::Const(x)),
            Op::Op(op) => all_monkeys.insert(id, Monkey::Op(op)),
        };
        Ok((input, ()))
    }

    pub fn parse_input(input: &str) -> IResult<&str, BTreeMap<MonkeyId, Monkey>> {
        let mut all_monkeys = BTreeMap::new();
        let (out, _) = all_consuming(terminated(
            separated_list1(line_ending, |i| monkey(i, &mut all_monkeys)),
            tuple((line_ending, eof)),
        ))(input)?;
        Ok((out, all_monkeys))
    }
}

mod part1 {
    use super::*;
    use crate::{data::Val, parse::parse_input};
    use advent_lib::parse::read_and_leak;

    pub fn run(file_name: &str) -> Result<Val, Box<dyn Error>> {
        let input = read_and_leak(file_name)?;
        let (_, monkeys) = parse_input(input)?;
        Ok(monkeys["root"].eval(&monkeys))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1 answer: {:#?}", part1::run("input.txt")?);
    // println!("Part 2 answer: {:#?}", part2::run("input.txt")?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1::run("inputtest.txt")?, 152);
        Ok(())
    }

    #[test]
    fn part1_ans() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1::run("input.txt")?, 31017034894002);
        Ok(())
    }

    // #[test]
    // fn test_part2() -> Result<(), Box<dyn Error>> {
    //     assert_eq!(part2::run("inputtest.txt")?, 1623178306);
    //     Ok(())
    // }

    // #[test]
    // fn part2_ans() -> Result<(), Box<dyn Error>> {
    //     assert_eq!(part2::run("input.txt")?, 4248669215955);
    //     Ok(())
    // }
}
