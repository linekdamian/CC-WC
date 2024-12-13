use std::{
    fs::File,
    io::{stdin, IsTerminal, Read, Result},
};

#[derive(Debug)]
pub struct WordCount {
    bytes: usize,
    lines: usize,
    words: usize,
    characters: usize,
    filename: String,
}

impl WordCount {
    pub fn init() -> WordCount {
        WordCount {
            bytes: 0,
            lines: 0,
            words: 0,
            characters: 0,
            filename: String::new(),
        }
    }

    pub fn set_filename(&mut self, filename: String) {
        self.filename = filename;
    }

    fn write(&mut self, buf: String, size: usize) {
        self.bytes = size;
        self.lines = buf.lines().count();
        self.words = buf.split_whitespace().count();
        self.characters = buf.char_indices().count();
    }

    pub fn calc(&mut self) -> Result<()> {
        let mut buf = String::new();
        let mut input = stdin();

        if !input.is_terminal() {
            match input.read_to_string(&mut buf) {
                Ok(size) => {
                    self.write(buf, size);
                    return Ok(());
                }
                _ => (),
            }
        }
        match File::open(&self.filename) {
            Ok(mut file) => match file.read_to_string(&mut buf) {
                Ok(size) => {
                    self.write(buf, size);
                    return Ok(());
                }
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
        }
    }

    fn print_spaces() {
        print!("{:3}", " ");
    }

    pub fn print(&mut self, arguments: &WordCountArguments) {
        WordCount::print_spaces();
        if arguments.lines {
            print!("{}", self.lines);
            WordCount::print_spaces();
        }
        if arguments.words {
            print!("{}", self.words);
            WordCount::print_spaces();
        }
        if arguments.characters {
            print!("{}", self.characters);
            WordCount::print_spaces();
        }
        if arguments.bytes {
            print!("{}", self.bytes);
            WordCount::print_spaces();
        }

        println!("{}", self.filename);
    }
}

#[derive(Debug)]
pub struct WordCountArguments {
    bytes: bool,
    lines: bool,
    words: bool,
    characters: bool,
}

impl WordCountArguments {
    pub fn init() -> WordCountArguments {
        WordCountArguments {
            bytes: false,
            lines: false,
            words: false,
            characters: false,
        }
    }

    pub fn set_argument(&mut self, arg: String) {
        if "-c" == arg {
            self.bytes = true;
        } else if "-l" == arg {
            self.lines = true;
        } else if "-w" == arg {
            self.words = true;
        } else if "-m" == arg {
            self.characters = true;
        }
    }

    pub fn set_default(&mut self) {
        self.bytes = true;
        self.lines = true;
        self.words = true;
    }
}
