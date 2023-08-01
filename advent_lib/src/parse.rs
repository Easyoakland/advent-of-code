use std::fs;
pub fn read_and_leak(file_path: &str) -> Result<&'static str, std::io::Error> {
    let input_str = fs::read_to_string(file_path)?;
    Ok(Box::leak(Box::new(input_str)))
}

pub mod nom {
    use nom::{
        character::complete::one_of,
        combinator::{opt, recognize},
        error::ParseError,
        sequence::tuple,
        AsChar, FindToken, IResult, InputIter, Parser,
    };
    use std::str::FromStr;

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
}

pub mod yap {
    use std::{borrow::Borrow, char, str::FromStr};
    use yap::Tokens;

    /// Attempt to parse a tag from an input.
    pub fn tag<Input, T>(input: &mut Input, tag: T) -> Option<T>
    where
        Input: Tokens,
        <Input as Tokens>::Item: PartialEq,
        T: IntoIterator + Clone,
        <T as IntoIterator>::Item: Borrow<<Input as Tokens>::Item>,
    {
        input.tokens(tag.clone()).then(|| tag)
    }

    /// Use [`str::parse`] to parse some amount of input after taking some input as dictated by a function.
    pub fn parse_from1<I, O, F>(
        input: &mut I,
        take_while: F,
    ) -> Option<Result<O, <O as FromStr>::Err>>
    where
        I: Tokens<Item = char>,
        O: FromStr,
        F: FnMut(&I::Item) -> bool,
    {
        let to_parse = input.tokens_while(take_while).collect::<String>();
        if to_parse.is_empty() {
            None
        } else {
            Some(to_parse.parse::<O>())
        }
    }

    /// Parses at least 1 digit.
    pub fn digit1<I, O>(input: &mut I) -> Option<Result<O, <O as FromStr>::Err>>
    where
        I: Tokens<Item = char>,
        O: FromStr,
    {
        let take_while = |t: &char| t.is_numeric();
        parse_from1(input, take_while)
    }

    /// Parses at least 1 digit with an optional sign (`+`/`-`) in front.
    pub fn signed_digit1<I, O>(input: &mut I) -> Option<Result<O, <O as FromStr>::Err>>
    where
        I: Tokens<Item = char>,
        O: FromStr + std::fmt::Debug,
        <O as FromStr>::Err: std::fmt::Debug,
    {
        let take_while = |&t: &char| t.is_numeric() || t == '+' || t == '-';
        parse_from1(input, take_while)
    }

    /// Parses at least 1 alphabetical character.
    pub fn alpha1<I, O>(input: &mut I) -> Option<Result<O, <O as FromStr>::Err>>
    where
        I: Tokens<Item = char>,
        O: FromStr,
    {
        let take_while = |t: &char| t.is_alphabetic();
        parse_from1(input, take_while)
    }

    /// Parses a line ending of either `\n` (like on linux)  or `\r\n` (like on windows)
    pub fn line_ending(tokens: &mut impl Tokens<Item = char>) -> Option<&str> {
        yap::one_of!(tokens;
            tokens.optional(|t| t.token('\n').then_some("\n")),
            tokens.optional(|t| t.tokens("\r\n".chars()).then_some("\r\n")),
        )
    }

    #[derive(Debug, thiserror::Error)]
    pub enum ParseError<Item> {
        #[error("Input wasn't fully consumed. Remainder: {0:?}")]
        AllConsuming(Vec<Item>),
    }

    /// Attempts to parse all remainder of input until next None. Consumes nothing on fail.
    pub fn all_consuming<'a, I, O, F>(input: &'a mut I, parser: F) -> Result<O, ParseError<I::Item>>
    where
        I: Tokens,
        F: FnOnce(&mut I) -> O + 'a,
    {
        let before_consuming = input.location();
        let res = parser(input);
        // Check nothing comes after
        match input.next() {
            None => Ok(res),
            Some(x) => {
                let res = Err(ParseError::AllConsuming(
                    std::iter::once(x).chain(input.as_iter()).collect(),
                ));
                input.set_location(before_consuming);
                res
            }
        }
    }
}
