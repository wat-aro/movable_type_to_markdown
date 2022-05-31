use clap::{Arg, Command};

fn main() {
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

    println!("FILE: {:?}", command.value_of("FILE"));
    println!("DIRECTORY: {:?}", command.value_of("DIRECTORY"));
}
