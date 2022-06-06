use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, not_line_ending, space0},
    error::ParseError,
    IResult,
};

pub fn key_value<'a, 'b, Error: ParseError<&'a str>>(
    key: &'b str,
) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, Error> + 'b {
    move |input: &'a str| {
        // preceeded_separate(tag(key), pair(tag(":"), space0), not_line_ending)(input)
        let (input, _) = tag(key)(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, _) = space0(input)?;
        let (input, output) = not_line_ending(input)?;
        line_ending(input).map(|(input, _)| (input, output))
    }
}
