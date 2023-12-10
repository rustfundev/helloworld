use say::say;

use clap::{Arg, Command};

fn main() {
    hello();
}

fn hello() {
    let matches = Command::new("hello")
        .version("1.0")
        .author("Tiago Mello")
        .about("A simple hello world program")
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .help("The name to greet"),
        )
        .get_matches();

    let message = {
        let name = matches.get_one::<String>("name").unwrap();
        format!("Hello, {name}!")
    };

    say(&message);
}
