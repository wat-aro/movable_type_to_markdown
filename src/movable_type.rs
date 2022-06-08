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
    metadata: Metadata<'a>,
    body: Body,
    comments: Vec<Comment<'a>>,
}

pub fn parse(input: &str) -> Result<Vec<Post>> {
    let result = posts(input);

    match result {
        Ok((_, posts)) => Ok(posts),
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
    let (input, metadata) = metadata(input).unwrap();
    let (input, body) = body(input).unwrap();
    let (input, comments) = comments(input).unwrap();
    let post = Post {
        metadata,
        body,
        comments,
    };
    Ok((input, post))
}
