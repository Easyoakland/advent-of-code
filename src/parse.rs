use crate::data::{Packet, Pair};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::all_consuming,
    error::ParseError,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, terminated},
    IResult, Parser,
};
use std::str::FromStr;

/// Takes a combinator and converts the output to a &str before parsing with `str.parse()`. Returns `nom::error::ErrorKind::Fail` on fail.
fn parse_from<'a, F, I, O, E>(mut f: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    F: Parser<I, I, E>,
    O: FromStr,
    I: Into<&'a str>,
    E: ParseError<I>,
{
    move |input: I| {
        let (input, out) = f.parse(input)?;
        let out = match (out.into()).parse::<O>() {
            Ok(x) => x,
            // If there is FromStr error either the output type can't be parsed from the original combinator's output.
            Err(_) => {
                return Err(nom::Err::Failure(E::from_error_kind(
                    input,
                    nom::error::ErrorKind::Fail,
                )));
            }
        };
        Ok((input, out))
    }
}

fn value(input: &str) -> IResult<&str, Packet> {
    let (input, x) = parse_from(digit1)(input)?;
    Ok((input, Packet::Integer(x)))
}

fn packet(input: &str) -> IResult<&str, Packet> {
    let (input, value) = delimited(
        tag("["),
        separated_list0(tag(","), alt((packet, value))),
        tag("]"),
    )(input)?;
    let x = value;
    let out = Packet::List(x);
    Ok((input, out))
}

fn pair(input: &str) -> IResult<&str, Pair> {
    let (input, left) = terminated(packet, line_ending)(input)?;
    let (input, right) = terminated(packet, line_ending)(input)?;
    Ok((input, Pair { left, right }))
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Pair>> {
    all_consuming(separated_list1(line_ending, pair))(input)
}
