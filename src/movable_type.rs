mod author;
mod body;
mod key_value;
mod metadata;

use metadata::Metadata;

use self::{
    body::{body, Body},
    metadata::metadata,
};

#[derive(Debug, PartialEq)]
pub struct Post<'a> {
    metadata: Metadata<'a>,
    body: Body,
}

pub fn parse(input: &str) -> Vec<Post> {
    let (input, metadata) = metadata(input).unwrap();
    let (_, body) = body(input).unwrap();
    let post = Post { metadata, body };
    vec![post]
}
