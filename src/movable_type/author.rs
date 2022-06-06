use nom::IResult;

use super::utils::key_value;

pub fn author(input: &str) -> IResult<&str, &str> {
    key_value("AUTHOR")(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn parse_author() -> Result<()> {
        let name = author("AUTHOR: wat-aro\n")?;
        assert_eq!(name, ("", "wat-aro"));
        Ok(())
    }
}
