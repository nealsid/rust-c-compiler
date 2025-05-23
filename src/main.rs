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
}
#[derive(Debug)]
enum Token {
    LeftParen = 0,
    RightParen = 1,
    Keyword = 2,
    LeftBrace = 3,
    RightBrace = 4,
}

struct TokenRegex {
    regex : Regex,
    token : Token,
}

fn next_token(buf : &str) -> Token {
    match buf {
        _ if Regex::new("^\\(").unwrap().is_match(buf) => Token::LeftParen,
        _ if Regex::new("^\\)").unwrap().is_match(buf) => Token::RightParen,
        _ if Regex::new("^\\{").unwrap().is_match(buf) => Token::LeftBrace,
        _ if Regex::new("^\\}").unwrap().is_match(buf) => Token::RightBrace,
        _ if Regex::new("^[A-z]+").unwrap().is_match(buf) => Token::Keyword,
        _ => Token::RightBrace,
    }
}

fn tokenize() {
    let tokens : Vec<Token> = Vec::new();
    let token_regexes : Vec<TokenRegex> = vec![
    TokenRegex { regex: Regex::new("^\\(").unwrap(), token: Token::LeftParen},
    TokenRegex { regex: Regex::new("^\\)").unwrap(), token: Token::RightParen }];
    
    
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
    dbg!("{}", next_token(&s));
    
}
