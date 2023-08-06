use crate::Cord;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::all_consuming,
    error::ParseError,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
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

/*
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
*/
fn cord<T: FromStr>(input: &str) -> IResult<&str, Cord<T>> {
    let (input, (x, y)) = separated_pair(parse_from(digit1), tag(","), parse_from(digit1))(input)?;
    Ok((input, Cord::from([x, y])))
}

fn cords<T: FromStr>(input: &str) -> IResult<&str, Vec<Cord<T>>> {
    separated_list1(tag(" -> "), cord)(input)
}

pub fn parse_input<T: FromStr>(input: &str) -> IResult<&str, Vec<Vec<Cord<T>>>> {
    all_consuming(terminated(separated_list1(line_ending, cords), line_ending))(input)
}
