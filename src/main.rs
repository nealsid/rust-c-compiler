use regex::Regex;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

struct Tokenizer {
    reg_ex_and_tokens : Vec<(Regex, Token)>
}

impl Tokenizer {
    fn new() -> Self {
        Tokenizer {
            reg_ex_and_tokens : vec![
            (Regex::new("^\\(").unwrap(), Token::LeftParen),
            (Regex::new("^\\)").unwrap(), Token::RightParen),
            (Regex::new("^\\{").unwrap(), Token::LeftBrace),
            (Regex::new("^\\}").unwrap(), Token::RightBrace),
            (Regex::new("^[A-z]+").unwrap(), Token::Keyword)
            ]
        }
    }

    fn tokenize(&self, buf : &str) -> Vec<Token> {
        let mut slice_start : usize = 0;
        let mut v : Vec<Token> = Vec::new();
        for reg_ex_and_token in & self.reg_ex_and_tokens {
            if reg_ex_and_token.0.is_match(&buf[slice_start..]) {
                v.push(reg_ex_and_token.1);
            }
        }

        v

    }
}
#[derive(Debug, Copy, Clone)]
enum Token {
    LeftParen = 0,
    RightParen = 1,
    Keyword = 2,
    LeftBrace = 3,
    RightBrace = 4,
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    
    let display = path.display();
    
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("Read {} successfully", args[1]),
    }
    let t = Tokenizer::new();
    let tokens = t.tokenize(&s);
    dbg!(tokens);
    
}
