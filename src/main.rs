use std::fs;

use anyhow::{Context, Result};
use clap::{Arg, Command};
use movable_type_to_markdown::movable_type;

fn main() -> Result<()> {
    let command = Command::new("Movable type to markdown")
        .author("wat-aro")
        .version("0.1.0")
        .about("Convert to markdown from movable type")
        .arg(
            Arg::new("FILE")
                .required(true)
                .help("Target movable type file location."),
        )
        .arg(
            Arg::new("DIRECTORY")
                .required(true)
                .help("Output directory"),
        )
        .get_matches();

    let filename = command
        .value_of("FILE")
        .context("No such file or directory")?;
    let contents = fs::read_to_string(filename)?;
    let posts = movable_type::parse(&contents)?;

    posts.iter().for_each(|post| {
        println!("TITLE: {}", post.metadata.title);
        // println!("BODY: {:?}", post.body.0);
    });
    println!("COUNT: {}", posts.len());
    println!("DIRECTORY: {:?}", command.value_of("DIRECTORY"));
    Ok(())
}
