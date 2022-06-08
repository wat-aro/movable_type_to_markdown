use chrono::{DateTime, Utc};
use nom::{
    branch::alt,
    bytes::{complete::take_until, streaming::tag},
    character::complete::{newline, u8},
    combinator::map,
    multi::separated_list0,
    sequence::{pair, tuple},
    IResult,
};

use super::common::{author, date};

#[derive(Debug, PartialEq)]
pub struct Comment<'a> {
    author: &'a str,
    ip: IP,
    date: DateTime<Utc>,
    body: &'a str,
}

#[derive(Debug, PartialEq)]
struct IP(u8, u8, u8, u8);

pub fn comments(input: &str) -> IResult<&str, Vec<Comment>> {
    let (input, _) = pair(tag("-----"), newline)(input)?;
    let (input, comments) = separated_list0(pair(tag("-----"), newline), comment)(input)?;
    let (input, _) = pair(tag("--------"), newline)(input)?;
    Ok((input, comments))
}

fn comment(input: &str) -> IResult<&str, Comment> {
    let (input, _) = tuple((tag("COMMENT"), tag(":"), newline))(input)?;
    let (input, author) = author(input)?;
    let (input, ip) = ip(input)?;
    let (input, date) = date(input)?;
    let (input, body) = body(input)?;
    Ok((
        input,
        Comment {
            author,
            ip,
            date,
            body,
        },
    ))
}

fn ip(input: &str) -> IResult<&str, IP> {
    let (input, _) = tag("IP: ")(input)?;
    let (input, ip) = map(
        tuple((u8, tag("."), u8, tag("."), u8, tag("."), u8)),
        |(i, _, j, _, k, _, l)| IP(i, j, k, l),
    )(input)?;
    let (input, _) = newline(input)?;
    Ok((input, ip))
}

fn body(input: &str) -> IResult<&str, &str> {
    alt((take_until("-----"), take_until("--------")))(input)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn parse_empty_comments() -> Result<()> {
        let comments = comments(
            r#"-----
--------
"#,
        )?;
        assert_eq!(comments, ("", vec![]));
        Ok(())
    }

    #[test]
    fn parse_comments() -> Result<()> {
        let comments = comments(
            r#"-----
COMMENT:
AUTHOR: wat-aro
IP: 127.0.0.1
DATE: 09/16/2021 22:09:33
これは
コメント
です
-----
COMMENT:
AUTHOR: wat-wat
IP: 196.168.1.1
DATE: 09/20/2021 22:10:00
これは
コメント2
です
--------
"#,
        )?;

        assert_eq!(
            comments,
            (
                "",
                vec![
                    Comment {
                        author: "wat-aro",
                        ip: IP(127, 0, 0, 1),
                        date: Utc.ymd(2021, 9, 16).and_hms(22, 9, 33),
                        body: "これは\nコメント\nです\n"
                    },
                    Comment {
                        author: "wat-wat",
                        ip: IP(196, 168, 1, 1),
                        date: Utc.ymd(2021, 9, 20).and_hms(22, 10, 0),
                        body: "これは\nコメント2\nです\n"
                    }
                ]
            )
        );
        Ok(())
    }

    #[test]
    fn parse_comment() -> Result<()> {
        let comment = comment(
            r#"COMMENT:
AUTHOR: wat-aro
IP: 127.0.0.1
DATE: 09/16/2021 22:09:33
これは
コメント
です
--------
"#,
        )?;
        assert_eq!(
            comment,
            (
                "--------\n",
                Comment {
                    author: "wat-aro",
                    ip: IP(127, 0, 0, 1),
                    date: Utc.ymd(2021, 9, 16).and_hms(22, 9, 33),
                    body: "これは\nコメント\nです\n"
                }
            )
        );
        Ok(())
    }

    #[test]
    fn parse_ip() -> Result<()> {
        let ip = ip(r#"IP: 127.0.0.1
"#)?;
        assert_eq!(ip, ("", IP(127, 0, 0, 1)));
        Ok(())
    }
}
