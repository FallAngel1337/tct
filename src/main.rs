use std::{env, process};
use std::error::Error;
use tct::cmd::COMMANDS;
use tct::cmd::man;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if &args[1] == "setup" {
        if  args.len() >= 3 {
            man::install_manpages(&args[2], "/opt/man")?;
            process::exit(0);
        }
        process::exit(1);
    }

    COMMANDS
        .get(args.get(1).unwrap_or_else(|| {
            eprintln!("need to pass an argument");
            process::exit(1);
        }).as_str())
        .unwrap_or_else(|| {
            eprintln!("{} isn't a valid command", args[1]);
            process::exit(1);
        })(&args[1..])
}
