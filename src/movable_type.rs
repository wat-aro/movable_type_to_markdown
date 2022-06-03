mod metadata;

use metadata::Metadata;

use self::metadata::metadata;

#[derive(Debug, PartialEq)]
pub struct Post<'a> {
    metadata: Metadata<'a>,
}

pub fn parse(input: &str) -> Vec<Post> {
    let (_, metadata) = metadata(input).unwrap();
    let post = Post { metadata };
    vec![post]
}
