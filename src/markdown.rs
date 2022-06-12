use chrono::{DateTime, Utc};

use crate::movable_type::Post;

#[derive(Debug, PartialEq)]
pub struct Markdown<'a> {
    title: &'a str,
    published: DateTime<Utc>,
    tags: Vec<&'a str>,
    body: &'a str,
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

impl<'a> Markdown<'a> {
    pub fn dump(&self) -> String {
        let mut string = String::new();
        string.push_str("---\n");
        string.push_str(&format!("title: \"{}\"\n", self.title));
        string.push_str(&format!(
            "published: {}\n",
            self.published.format("%Y/%m/%d").to_string()
        ));
        if self.tags.len() > 0 {
            string.push_str(&format!("tags:\n"));
            self.tags.iter().for_each(|tag| {
                string.push_str(&format!("  - {}\n", tag));
            })
        }
        string.push_str("---\n\n");
        string.push_str(&format!("{}\n", self.body));
        string
    }

    pub fn path(&self, directory: &str) -> String {
        format!("{}/{}.md", directory, self.title)
    }
}
