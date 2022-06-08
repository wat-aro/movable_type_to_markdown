use super::common::{author, date, key_value};
use chrono::{DateTime, Utc};
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct Metadata<'a> {
    author: &'a str,
    pub title: &'a str,
    basename: &'a str,
    status: &'a str,
    allow_comments: bool,
    convert_breaks: bool,
    date: DateTime<Utc>,
    image: Option<&'a str>,
    category: Vec<&'a str>,
}

pub fn metadata(input: &str) -> IResult<&str, Metadata> {
    let (input, author) = author(input)?;
    let (input, title) = title(input)?;
    let (input, basename) = basename(input)?;
    let (input, status) = status(input)?;
    let (input, allow_comments) = allow_comments(input)?;
    let (input, convert_breaks) = convert_breaks(input)?;
    let (input, date) = date(input)?;
    let (input, category) = category(input)?;
    let (input, image) = image(input)?;

    Ok((
        input,
        Metadata {
            author,
            title,
            basename,
            status,
            allow_comments,
            convert_breaks,
            date,
            category,
            image,
        },
    ))
}

fn title(input: &str) -> IResult<&str, &str> {
    key_value("TITLE")(input)
}

fn basename(input: &str) -> IResult<&str, &str> {
    key_value("BASENAME")(input)
}

fn status(input: &str) -> IResult<&str, &str> {
    key_value("STATUS")(input)
}

fn allow_comments(input: &str) -> IResult<&str, bool> {
    map(key_value("ALLOW COMMENTS"), |str| {
        str.parse::<u8>().unwrap() != 0
    })(input)
}

fn convert_breaks(input: &str) -> IResult<&str, bool> {
    map(key_value("CONVERT BREAKS"), |str| {
        str.parse::<u8>().unwrap() != 0
    })(input)
}

fn category(input: &str) -> IResult<&str, Vec<&str>> {
    many0(key_value("CATEGORY"))(input)
}

fn image(input: &str) -> IResult<&str, Option<&str>> {
    opt(key_value("IMAGE"))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use chrono::{TimeZone, Utc};

    #[test]
    fn parse_title() -> Result<()> {
        let title = title("TITLE: Title\n")?;
        assert_eq!(title, ("", "Title"));
        Ok(())
    }

    #[test]
    fn parse_basename() -> Result<()> {
        let basename = basename("BASENAME: Basename\n")?;
        assert_eq!(basename, ("", "Basename"));
        Ok(())
    }

    #[test]
    fn parse_status() -> Result<()> {
        let status = status("STATUS: Publish\n")?;
        assert_eq!(status, ("", "Publish"));
        Ok(())
    }

    #[test]
    fn parse_allow_comments_true() -> Result<()> {
        let allow_comments = allow_comments("ALLOW COMMENTS: 1\n")?;
        assert_eq!(allow_comments, ("", true));
        Ok(())
    }

    #[test]
    fn parse_allow_comments_false() -> Result<()> {
        let allow_comments = allow_comments("ALLOW COMMENTS: 0\n")?;
        assert_eq!(allow_comments, ("", false));
        Ok(())
    }

    #[test]
    fn parse_convert_breaks_true() -> Result<()> {
        let convert_breaks = convert_breaks("CONVERT BREAKS: 1\n")?;
        assert_eq!(convert_breaks, ("", true));
        Ok(())
    }

    #[test]
    fn parse_convert_breaks_false() -> Result<()> {
        let convert_breaks = convert_breaks("CONVERT BREAKS: 0\n")?;
        assert_eq!(convert_breaks, ("", false));
        Ok(())
    }

    #[test]
    fn parse_image() -> Result<()> {
        let image = image("IMAGE: http://example.com/image.jpg\n")?;
        assert_eq!(image, ("", Some("http://example.com/image.jpg")));
        Ok(())
    }

    #[test]
    fn parse_metadata() -> Result<()> {
        let metadata = metadata(
            "AUTHOR: wat-aro
TITLE: Title
BASENAME: Basename
STATUS: Publish
ALLOW COMMENTS: 1
CONVERT BREAKS: 0
DATE: 09/16/2021 22:09:33
IMAGE: http://example.com/image.jpg
",
        )?;
        assert_eq!(
            metadata,
            (
                "",
                Metadata {
                    author: "wat-aro",
                    title: "Title",
                    basename: "Basename",
                    status: "Publish",
                    allow_comments: true,
                    convert_breaks: false,
                    date: Utc.ymd(2021, 9, 16).and_hms(22, 9, 33),
                    category: vec![],
                    image: Some("http://example.com/image.jpg")
                }
            )
        );
        Ok(())
    }
}
