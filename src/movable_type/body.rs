use html_parser::{Dom, Element, Node};
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
        let children: Vec<String> = self.0.children.iter().map(|node| dump_node(node)).collect();
        children.join("")
    }
}

fn dump_node(node: &Node) -> String {
    match node {
        Node::Text(text) => text.to_string(),
        Node::Element(element) => dump_element(element),
        Node::Comment(_) => String::from(""),
    }
}

fn dump_element(element: &Element) -> String {
    match &element.name[..] {
        "p" => {
            let children: Vec<String> = dump_children(&element.children);
            format!("{}\n\n", children.join(""))
        }
        "br" => "  ".to_string(),
        "a" => {
            let text = dump_children(&element.children).join("");
            match element.attributes.get("href") {
                Some(link) => {
                    let link = match link {
                        Some(link) => link,
                        None => "",
                    };
                    format!("[{text}]({link})", text = text, link = link)
                }
                None => format!("[{text}]()", text = text),
            }
        }
        _ => todo!(),
    }
}

fn dump_children(children: &Vec<Node>) -> Vec<String> {
    children.iter().map(|node| dump_node(node)).collect()
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

    impl Body {
        fn initialize_from_html(html: &str) -> Self {
            let dom = Dom::parse(html).unwrap();
            Self(dom)
        }
    }

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
        let body = Body::initialize_from_html("Hello");
        assert_eq!(body.dump(), "Hello");
        Ok(())
    }

    #[test]
    fn dump_comment() -> Result<()> {
        let body = Body::initialize_from_html("<!-- comment -->");
        assert_eq!(body.dump(), "");
        Ok(())
    }

    #[test]
    fn dump_p() -> Result<()> {
        let body = Body::initialize_from_html("<p>paragraph</p>");
        assert_eq!(body.dump(), "paragraph\n\n");
        Ok(())
    }

    #[test]
    fn dump_br() -> Result<()> {
        let body = Body::initialize_from_html("Hello<br/>");
        assert_eq!(body.dump(), "Hello  ");
        Ok(())
    }

    #[test]
    fn dump_a() -> Result<()> {
        let body = Body::initialize_from_html("<a href=\"http://example.com\">Link</a>");
        assert_eq!(body.dump(), "[Link](http://example.com)");
        Ok(())
    }
}
