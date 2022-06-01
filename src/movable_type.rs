use std::error::Error;

use chrono::{DateTime, FixedOffset};
use nom::{
    bytes::streaming::tag, character::complete::anychar, combinator::recognize, multi::many1,
    sequence::preceded, IResult,
};

#[derive(Debug, PartialEq)]
pub struct Post {
    meta_data: MetaData,
}

#[derive(Debug, PartialEq)]
pub struct MetaData {
    author: String,
    title: String,
    basename: String,
    allow_comments: bool,
    convert_breaks: bool,
    date: DateTime<FixedOffset>,
    image: String,
}

pub fn parse(_text: &str) -> Vec<Post> {
    vec![]
}

fn author(input: &str) -> IResult<&str, &str> {
    preceded(tag("AUTHOR: "), recognize(many1(anychar)))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn parse_author() -> Result<()> {
        let name = author("AUTHOR: wat-aro")?;
        assert_eq!(name, ("", "wat-aro"));
        Ok(())
    }
}
