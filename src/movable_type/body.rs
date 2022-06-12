use nom::{
    bytes::complete::{tag, take_until},
    character::complete::newline,
    combinator::map,
    sequence::{pair, preceded},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Body<'a>(pub &'a str);

pub fn body<'a>(input: &str) -> IResult<&str, Body> {
    let (input, _) = pair(tag("-----"), newline)(input)?;
    let (input, body) = map(
        preceded(pair(tag("BODY:"), newline), take_until("\n-----\n")),
        |str| Body(str),
    )(input)?;
    let (input, _) = newline(input)?;
    Ok((input, body))
}

#[cfg(test)]
mod tests {
    use crate::movable_type::comments::comments;

    use super::*;
    use anyhow::Result;

    #[test]
    fn parse_body() -> Result<()> {
        assert!(body(
            r#"-----
BODY:
<p><a class="keyword" href="http://example.com">Example</a>Lorem</a>Lorem ipsum dolor sit amet, consectetur adipiscing elit</p>

<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.<br />
--------------------
-----
        "#
        ).is_ok());

        Ok(())
    }

    #[test]
    fn parse_comments_after_body() -> Result<()> {
        let text = r#"-----
BODY:
<p><a class="keyword" href="http://example.com">Example</a>Lorem</a>Lorem ipsum dolor sit amet, consectetur adipiscing elit</p>

<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.<br />
--------------------
-----
--------
"#;

        let (input, _) = body(text)?;
        let (_, _) = comments(input)?;

        Ok(())
    }
}
