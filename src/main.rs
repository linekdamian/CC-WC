use std::io::Error;
use std::{env, process};
use wc::{WordCount, WordCountArguments};

pub mod wc;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let mut arguments = WordCountArguments::init();
    let mut wc = WordCount::init();

    let mut iter = args.iter();
    iter.next();

    for arg in iter {
        if arg.starts_with("-") {
            arguments.set_argument(arg);
            continue;
        }

        wc.set_filename(arg);
    }

    match wc.calc(&arguments) {
        Ok(_) => wc.print(&arguments),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    Ok(())
}
