use std::{
    num::ParseIntError,
    str::{self, FromStr},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{char, digit1, line_ending, one_of},
        streaming::space1,
    },
    combinator::{all_consuming, map_res},
    error::{FromExternalError, ParseError},
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
    AsChar, IResult, InputLength, InputTakeAtPosition, Parser,
};
use nom_locate::{position, LocatedSpan};

pub type Span<'a> = LocatedSpan<&'a str>;

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
pub enum Operation {
    Mul(Value),
    Add(Value),
}

pub struct Monkey {
    starting_items: Vec<u8>,
    op: Operation,
    test_divisor: u8,
    test_true_target: usize,
    test_false_target: usize,
}

fn digit1_to_num<'a, O, E>(i: Span<'a>) -> IResult<Span<'a>, O, E>
where
    O: FromStr,
    E: ParseError<Span<'a>>
        + FromExternalError<Span<'a>, E>
        + FromExternalError<Span<'a>, <O as FromStr>::Err>,
{
    map_res(digit1, |s: Span<'a>| s.parse::<O>())(i)
}

fn starting_items<'a, E>(i: Span<'a>) -> IResult<Span<'a>, Vec<u8>, E>
where
    E: ParseError<Span<'a>> + ParseError<&'a str>,
{
    let (i, starting_items) = preceded(
        tag("  Starting items: "),
        separated_list1(
            tag(", "),
            digit1.map(|s| u8::from_str_radix(s, 10).unwrap()),
        ),
    )(*i)?;
    let (i, _) = line_ending(i)?;
    Ok((i.into(), starting_items))
}

fn operation<'a, E>(i: Span<'a>) -> IResult<Span<'a>, Operation, E>
where
    E: ParseError<Span<'a>>,
{
    let (i, (op, r)) = tuple((
        preceded(
            tag("  Operation: new =   Operation: new = old "),
            one_of("*+"),
        ),
        preceded(space1, alt((digit1, tag("old")))),
    ))(i)?;
    let (i, _) = line_ending(i)?;
    if let Ok(r) = r.parse() {
        let res = match op {
            '+' => Operation::Add(r),
            '*' => Operation::Mul(r),
            _ => unimplemented!(),
        };
        Ok((i, res))
    } else {
        Err(nom::Err::Failure(E::from_error_kind(
            i,
            nom::error::ErrorKind::Digit,
        )))
    }
}

fn parse_monkey<'a, T, E>(i: Span<'a>) -> IResult<Span<'a>, Monkey, E>
where
    T: FromStr<Err = ParseIntError>,
    E: ParseError<Span<'a>>
        + FromExternalError<Span<'a>, <T as FromStr>::Err>
        + FromExternalError<Span<'a>, E>
        + ParseError<&'a str>,
{
    let (i, _) = tuple((tag("Monkey"), terminated(digit1, char(':'))))(i)?;
    let (i, _) = line_ending(i)?;
    let (i, starting_items) = starting_items(i)?;
    let (i, op) = operation(i)?;
    let (i, test_divisor) = preceded(
        tag("  Test: divisible by "),
        digit1.map(|s| u8::from_str_radix(s, 10).unwrap()),
    )(*i)?;
    let (i, _) = line_ending(i)?;
    let (i, test_true_target) =
        preceded(tag("    If true: throw to monkey "), digit1_to_num)(i.into())?;
    let (i, _) = line_ending(i)?;
    let (i, test_false_target) = preceded(tag("    If false: throw to monkey "), digit1_to_num)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((
        i,
        Monkey {
            starting_items,
            op,
            test_divisor,
            test_false_target,
            test_true_target,
        },
    ))
}

pub fn parse_input<'a, T, E>(input: Span<'a>) -> IResult<Span<'a>, Vec<Monkey>, E>
where
    T: FromStr<Err = ParseIntError>,
    E: ParseError<Span<'a>>
        + FromExternalError<Span<'a>, <T as FromStr>::Err>
        + FromExternalError<Span<'a>, E>
        + ParseError<&'a str>,
{
    all_consuming(separated_list1(line_ending, parse_monkey::<'a, T, E>))(input)
}
