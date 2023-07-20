use nom::{
    character::complete::one_of,
    combinator::{opt, recognize},
    error::ParseError,
    sequence::tuple,
    AsChar, FindToken, IResult, InputIter, Parser,
};
use std::{fs, str::FromStr};

pub fn read_and_leak(file_path: &str) -> Result<&'static str, std::io::Error> {
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

/// Parses a `+` or `-` symbol preceding the parser that may or may not be preceded by a `-`.
pub fn signed<'a, F, I, E>(f: F) -> impl FnMut(I) -> IResult<I, I, E> + 'a
where
    F: Parser<I, I, E> + 'a,
    I: Into<&'a str>
        + Clone
        + nom::Slice<std::ops::RangeFrom<usize>>
        + nom::InputIter
        + nom::Offset
        + nom::Slice<std::ops::RangeTo<usize>>
        + 'a,
    E: ParseError<I> + 'a,
    <I as InputIter>::Item: AsChar + Copy,
    &'a str: FindToken<<I as InputIter>::Item>,
{
    recognize(tuple((opt(one_of("-+")), f)))
}

mod test {
    // For some reason the imports are indicated as unused but they aren't.
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use nom::character::complete::digit1;
    #[allow(unused_imports)]
    use nom::error::Error;

    #[test]
    fn parse_from_test() {
        let input = "1234";
        assert_eq!(
            parse_from::<_, _, u32, Error<_>>(digit1)(input),
            Ok(("", 1234))
        );
    }

    #[test]
    fn parse_from_signed_test() {
        let input = "-1234";
        assert_eq!(
            parse_from::<_, _, i32, Error<_>>(signed(digit1))(input),
            Ok(("", -1234))
        );
        let input = "+1234";
        assert_eq!(
            parse_from::<_, _, u32, Error<_>>(signed(digit1))(input),
            Ok(("", 1234))
        );
    }
}
