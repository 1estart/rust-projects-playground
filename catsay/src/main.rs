extern crate colored;
extern crate structopt;

use colored::*;
use exitfailure::ExitFailure;
use failure::ResultExt;
use structopt::StructOpt;

use std::io::{self, Read};

mod options;
use options::*;

fn main() -> Result<(), ExitFailure> {
    let options = Options::from_args();
    let mut message = String::new();
    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
        message = message.trim().to_string();
    } else {
        message = options.message;
    };
    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.")
    }
    let eye = if options.dead { "x" } else { "o" };
    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)
                .with_context(|_| format!("could not read file {:?}", path))?;
            let cat_picture = cat_template.replace("{eye}", eye);
            println!("{}", &cat_picture);
        }
        None => {
            println!("{}", message.bright_yellow().underline().on_purple());
            println!(" \\");
            println!("  \\");
            println!("     /\\_/\\");
            println!("    ( {eye} {eye} )", eye = eye);
            println!("    =( I )=");
        }
    };
    Ok(())
}
