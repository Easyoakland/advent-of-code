use std::fs;

use nom::{error::ParseError, IResult, Parser};
use std::str::FromStr;

pub fn read_file_static(file_path: &str) -> Result<&'static str, std::io::Error> {
    let input_str = fs::read_to_string(file_path)?;
    Ok(Box::leak(Box::new(input_str)))
}

/// Takes a combinator and converts the output to a &str before parsing with `str.parse()`. Returns `nom::error::ErrorKind::Fail` on fail.
pub fn parse_from<'a, F, I, O, E>(mut f: F) -> impl FnMut(I) -> IResult<I, O, E>
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
