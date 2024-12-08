use std::{fs::File, io::Error};

#[derive(Debug)]
pub struct WordCount {
    bytes: u64,
    filename: String,
}

impl WordCount {
    pub fn init() -> WordCount {
        WordCount {
            bytes: 0,
            filename: String::new(),
        }
    }

    pub fn set_filename(&mut self, filename: &String) {
        self.filename = filename.clone();
    }

    pub fn calc(&mut self, arguments: &WordCountArguments) -> Result<(), Error> {
        match File::open(&self.filename) {
            Ok(file) => {
                if arguments.bytes {
                    self.bytes = file.metadata().unwrap().len()
                }
                Ok(())
            }
            Err(e) => return Err(e),
        }
    }

    pub fn print(&mut self, arguments: &WordCountArguments) {
        if arguments.bytes {
            print!("{}\t", self.bytes)
        }

        println!("{}", self.filename)
    }
}

#[derive(Debug)]
pub struct WordCountArguments {
    bytes: bool,
}

impl WordCountArguments {
    pub fn init() -> WordCountArguments {
        WordCountArguments { bytes: false }
    }

    pub fn set_argument(&mut self, arg: &str) {
        if "-c" == arg {
            self.bytes = true;
        }
    }
}
