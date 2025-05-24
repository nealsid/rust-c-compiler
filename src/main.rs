use regex::Regex;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

struct RegExAndToken {
    regex: Regex,
    token: Token,
}

struct Tokenizer {
    reg_ex_and_tokens: Vec<RegExAndToken>,
}

impl Tokenizer {
    fn new() -> Self {
        Tokenizer {
            reg_ex_and_tokens: vec![
                RegExAndToken {
                    regex: Regex::new("^\\(").unwrap(),
                    token: Token::LeftParen,
                },
                RegExAndToken {
                    regex: Regex::new("^\\)").unwrap(),
                    token: Token::RightParen,
                },
                RegExAndToken {
                    regex: Regex::new("^\\{").unwrap(),
                    token: Token::LeftBrace,
                },
                RegExAndToken {
                    regex: Regex::new("^\\}").unwrap(),
                    token: Token::RightBrace,
                },
                RegExAndToken {
                    regex: Regex::new("^[A-z]+").unwrap(),
                    token: Token::Keyword,
                },
            ],
        }
    }

    fn tokenize(&self, buf: &str) -> Vec<Token> {
        let mut slice_start: usize = 0;
        let mut v: Vec<Token> = Vec::new();
        while slice_start < buf.len() {
            for reg_ex_and_token in &self.reg_ex_and_tokens {
                let next_token = reg_ex_and_token.regex.find(&buf[slice_start..]);
                match next_token {
                    Some(hit) => {
                        println!("Matched {}", reg_ex_and_token.regex);
                        v.push(reg_ex_and_token.token);
                        slice_start = hit.end();
                        while buf.as_bytes()[slice_start] == b' ' {
                            slice_start += 1;
                        }
                    }
                    None => continue,
                }
            }
            panic!("Invalid token: \"{}\"", buf.as_bytes()[slice_start] as char);
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
