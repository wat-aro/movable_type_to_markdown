#[derive(Debug, PartialEq)]
pub struct Post {}

pub fn parse(_text: &str) -> Vec<Post> {
    vec![Post {}]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_str() {
        assert_eq!(parse(""), vec![Post {}]);
    }
}
