use html_parser::Dom;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::newline,
    combinator::map_res,
    sequence::{pair, preceded},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Body(pub Dom);

pub fn body<'a>(input: &str) -> IResult<&str, Body> {
    let (input, _) = pair(tag("-----"), newline)(input)?;
    map_res(
        preceded(pair(tag("BODY:"), newline), take_until("-----")),
        |str| Dom::parse(str).map(|dom| Body(dom)),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn parse_body() -> Result<()> {
        assert!(body(
            r#"-----
BODY:
        <p><a class="keyword" href="http://example.com">Example</a>Lorem</a>Lorem ipsum dolor sit amet, consectetur adipiscing elit</p>

        <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.<br />
        -----
        "#
        ).is_ok());

        Ok(())
    }
}
