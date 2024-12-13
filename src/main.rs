use std::io::Error;
use std::{env, process};
use wc::{WordCount, WordCountArguments};

pub mod wc;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut arguments = WordCountArguments::init();
    let mut wc = WordCount::init();
    let mut is_default = true;

    for arg in args {
        if arg.starts_with("-") {
            arguments.set_argument(arg);
            is_default = false;
            continue;
        }

        wc.set_filename(arg);
    }

    if is_default {
        arguments.set_default();
    }

    match wc.calc() {
        Ok(_) => wc.print(&arguments),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    Ok(())
}
