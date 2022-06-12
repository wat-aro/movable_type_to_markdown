use std::{
    fs::{self, File},
    io::Write,
};

use anyhow::{Context, Result};
use clap::{Arg, ArgMatches, Command};
use movable_type_to_markdown::{markdown::Markdown, movable_type};

fn main() -> Result<()> {
    let command = build_command();

    let filename = command
        .value_of("FILE")
        .context("No such file or directory")?;
    let output_directory = command
        .value_of("DIRECTORY")
        .context("Required directory")?;
    let contents = fs::read_to_string(filename)?;
    let posts = movable_type::parse(&contents)?;

    posts.into_iter().for_each(|post| {
        let markdown = Markdown::from(post);
        let mut file = File::create(markdown.path(output_directory)).unwrap();
        let content = markdown.dump();
        write!(file, "{}", content).unwrap();
    });
    Ok(())
}

fn build_command() -> ArgMatches {
    Command::new("Movable type to markdown")
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
        .get_matches()
}
