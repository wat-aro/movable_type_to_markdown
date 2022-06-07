use chrono::{DateTime, TimeZone, Utc};
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, not_line_ending, space0},
    combinator::map,
    error::ParseError,
    IResult,
};

pub fn author(input: &str) -> IResult<&str, &str> {
    key_value("AUTHOR")(input)
}

pub fn date(input: &str) -> IResult<&str, DateTime<Utc>> {
    map(key_value("DATE"), |str| {
        Utc.datetime_from_str(str, "%m/%d/%Y %H:%M:%S").unwrap()
        // str.parse::<DateTime<Utc>>().unwrap()
    })(input)
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn parse_author() -> Result<()> {
        let name = author("AUTHOR: wat-aro\n")?;
        assert_eq!(name, ("", "wat-aro"));
        Ok(())
    }

    #[test]
    fn parse_date_time() -> Result<()> {
        let date = date("DATE: 09/16/2021 22:09:33\n")?;
        assert_eq!(date, ("", Utc.ymd(2021, 9, 16).and_hms(22, 9, 33)));
        Ok(())
    }
}
