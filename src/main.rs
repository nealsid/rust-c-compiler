use regex::Regex;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

enum Token {
    LeftParen,
    RightParen,
    Keyword,
    LeftBrace,
}

struct TokenRegex {
    regex : Regex,
    token : Token,
}


fn tokenize() {
    let token_regexes : Vec<TokenRegex> = vec![
        TokenRegex { regex: Regex::new("^(").unwrap(), token: Token::LeftParen},
        TokenRegex { regex: Regex::new("^)").unwrap(), token: Token::RightParen }];
    

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

    
}
