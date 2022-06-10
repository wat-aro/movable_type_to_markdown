use chrono::{DateTime, Utc};
use html_parser::Dom;

use crate::movable_type::Post;

#[derive(Debug, PartialEq)]
struct Markdown<'a> {
    title: &'a str,
    published: DateTime<Utc>,
    tags: Vec<&'a str>,
    body: Dom,
}

impl<'a> From<Post<'a>> for Markdown<'a> {
    fn from(post: Post<'a>) -> Self {
        let title = post.metadata.title;
        let published = post.metadata.date;
        let tags = post.metadata.category;
        let body = post.body.0;

        Markdown {
            title,
            published,
            tags,
            body,
        }
    }
}
