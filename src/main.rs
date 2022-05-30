use clap::Command;

fn main() {
    Command::new("Movable type to markdown")
        .author("wat-aro")
        .version("0.1.0")
        .about("Convert to markdown from movable type")
        .get_matches();

    println!("hello");
}
