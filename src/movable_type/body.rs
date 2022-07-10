use html_parser::{Dom, Node};
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::newline,
    combinator::map_res,
    sequence::{pair, preceded},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Body(Dom);

impl Body {
    pub fn new(dom: Dom) -> Self {
        Self(dom)
    }

    pub fn dump(&self) -> String {
        let children: Vec<String> = self
            .0
            .children
            .iter()
            .map(|child| dump_node(child))
            .collect();
        children.join("\n")
    }
}

fn dump_node(node: &Node) -> String {
    match node {
        Node::Text(text) => text.to_string(),
        Node::Element(_) => todo!(),
        Node::Comment(_) => todo!(),
    }
}

pub fn body<'a>(input: &str) -> IResult<&str, Body> {
    let (input, _) = pair(tag("-----"), newline)(input)?;
    let (input, body) = map_res(
        preceded(pair(tag("BODY:"), newline), take_until("\n-----\n")),
        |str| Dom::parse(str).map(|dom| Body(dom)),
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

    #[test]
    fn dump_text() -> Result<()> {
        let dom = Dom::parse("Hello")?;
        let body = Body::new(dom);
        assert_eq!(body.dump(), "Hello");
        Ok(())
    }
}
