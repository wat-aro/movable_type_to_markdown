mod body;
mod comments;
mod common;
mod metadata;

use metadata::Metadata;

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

pub fn parse(input: &str) -> Vec<Post> {
    let (input, metadata) = metadata(input).unwrap();
    let (input, body) = body(input).unwrap();
    let (_, comments) = comments(input).unwrap();
    let post = Post {
        metadata,
        body,
        comments,
    };
    vec![post]
}
