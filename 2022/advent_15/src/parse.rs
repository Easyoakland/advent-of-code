use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{all_consuming, opt, recognize},
    error::ParseError,
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
    IResult, Parser,
};
use num::Signed;
use std::str::FromStr;

use crate::{cord::Cord, data::Pair};
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
        let Ok(out) = (out.into()).parse::<O>()
        else {
            // If there is FromStr error the output type can't be parsed from the original combinator's output.
            return Err(nom::Err::Failure(E::from_error_kind(
                input,
                nom::error::ErrorKind::Fail,
            )));
        };
        Ok((input, out))
    }
}
//-2
//2
fn signed_digit1(input: &str) -> IResult<&str, &str> {
    recognize(tuple((opt(tag("-")), digit1)))(input)
}

//x=2, y=18
fn cord<T: FromStr + Signed>(input: &str) -> IResult<&str, Cord<T>> {
    let (input, x) = preceded(tag("x="), parse_from(signed_digit1))(input)?;
    let (input, y) = preceded(tag(", y="), parse_from(signed_digit1))(input)?;
    Ok((input, Cord(x, y)))
}

//Sensor at x=2, y=18: closest beacon is at x=-2, y=15
fn line<T: FromStr + Signed>(input: &str) -> IResult<&str, Pair<T>> {
    let (input, sensor) = preceded(tag("Sensor at "), cord)(input)?;
    let (input, beacon) = preceded(tag(": closest beacon is at "), cord)(input)?;
    Ok((input, Pair { sensor, beacon }))
}

pub fn parse_input<T: FromStr + Signed>(input: &str) -> IResult<&str, Vec<Pair<T>>> {
    all_consuming(terminated(separated_list1(line_ending, line), line_ending))(input)
}
