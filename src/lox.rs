use crate::scanner::token::Token;
use std::io::{Read, Write};
use std::process::exit;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Lox {
        Lox { had_error: false }
    }

    pub fn run_file(&self, path: &String) {
        use std::fs::File;
        use std::path::Path;
        let path = Path::new(&path);

        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(), why),
            Ok(file) => file,
        };

        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Err(why) => panic!("cannot read {}: {}", path.display(), why),
            Ok(_) => self.run(content),
        }
    }

    pub fn run_prompt(&mut self) {
        use std::io::{stdin, stdout};
        loop {
            print!("> ");
            let _ = stdout().flush();
            let mut input = String::new();

            let _ = stdin().read_line(&mut input);
            // NOTE: Enter(1) or Control-D (0)
            if input.len() <= 1 {
                break;
            }
            self.run(input);
            self.had_error = false;
        }
    }

    fn run(&self, source: String) {
        use crate::scanner::*;
        let mut scanner = Scanner::new(source);
        // TODO: Remove this debug after scan_tokens is (partly) implemented:w
        //scanner.print_source();

        let tokens: &Vec<Token> = scanner.scan_tokens();
        for token in tokens {
            println!("{:?}", token);
        }
        if self.had_error {
            exit(65)
        }
    }

    #[allow(dead_code)]
    fn error(&mut self, line: i32, message: String) {
        self.report(line, "".to_string(), message);
    }

    #[allow(dead_code)]
    fn report(&mut self, line: i32, location: String, message: String) {
        println!("[line {}] Error{}: {}", line, location, message);
        self.had_error = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_error() {
        let mut lox = Lox::new();

        assert_eq!(lox.had_error, false);
        lox.error(42, String::from("This is error"));
        assert_eq!(lox.had_error, true);
    }
}
