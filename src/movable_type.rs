mod body;
mod comments;
mod common;
mod metadata;

use anyhow::Result;
use metadata::Metadata;
use nom::{multi::many0, IResult};

use self::{
    body::{body, Body},
    comments::{comments, Comment},
    metadata::metadata,
};

#[derive(Debug, PartialEq)]
pub struct Post<'a> {
    pub metadata: Metadata<'a>,
    pub body: Body<'a>,
    pub comments: Vec<Comment<'a>>,
}

pub fn parse(input: &str) -> Result<Vec<Post>> {
    let result = posts(input);

    match result {
        Ok((input, posts)) => {
            if input != "" {
                println!("INPUT: {}", input);
            }
            Ok(posts)
        }
        Err(e) => {
            eprintln!("{}", e);
            Err(anyhow::anyhow!("Failed parsing."))
        }
    }
}

fn posts(input: &str) -> IResult<&str, Vec<Post>> {
    many0(post)(input)
}

fn post(input: &str) -> IResult<&str, Post> {
    let (input, metadata) = metadata(input)?;
    let (input, body) = body(input)?;
    let (input, comments) = comments(input)?;
    let post = Post {
        metadata,
        body,
        comments,
    };
    Ok((input, post))
}

#[cfg(test)]
mod tests {
    use crate::movable_type::comments::IP;

    use super::*;
    use anyhow::Result;
    use chrono::{TimeZone, Utc};

    #[test]
    fn parse_post() -> Result<()> {
        let post = post(
            r#"AUTHOR: wat-aro
TITLE: タイトル
BASENAME: 2018/08/07/203114
STATUS: Publish
ALLOW COMMENTS: 1
CONVERT BREAKS: 0
DATE: 08/07/2018 20:31:14
-----
BODY:
<p>test</p>

-----
COMMENT:
AUTHOR: anonymous
IP: 192.168.1.1
DATE: 02/28/2019 04:17:42
コメント!コメコメ!
-----
--------
"#,
        )?;
        assert_eq!(
            post,
            (
                "",
                Post {
                    metadata: Metadata {
                        author: "wat-aro",
                        title: "タイトル",
                        basename: "2018/08/07/203114",
                        status: "Publish",
                        allow_comments: true,
                        convert_breaks: false,
                        image: None,
                        category: vec![],
                        date: Utc.ymd(2018, 8, 7).and_hms(20, 31, 14)
                    },
                    body: Body("<p>test</p>\n"),
                    comments: vec![Comment {
                        author: "anonymous",
                        ip: IP::new(192, 168, 1, 1),
                        date: Utc.ymd(2019, 2, 28).and_hms(4, 17, 42),
                        body: "コメント!コメコメ!\n"
                    }]
                }
            )
        );
        Ok(())
    }
}
