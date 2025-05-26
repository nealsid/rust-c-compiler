use regex::Regex;

struct RegExAndToken {
    regex: Regex,
    token: Token,
}

#[derive(Debug, Copy, Clone)]
pub enum Token {
    LeftParen = 0,
    RightParen = 1,
    Keyword = 2,
    LeftBrace = 3,
    RightBrace = 4,
    Comma = 5,
    Asterisk = 6,
    LeftBracket = 7,
    RightBracket = 8,
    NumericConstant = 9,
    Semicolon = 10,
}

pub struct Tokenizer {
    reg_ex_and_tokens: Vec<RegExAndToken>,
}

impl Tokenizer {
    pub fn new() -> Self {
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
                regex: Regex::new("^[A-Za-z_]+").unwrap(),
                token: Token::Keyword,
            },
            RegExAndToken {
                regex: Regex::new("^,").unwrap(),
                token: Token::Comma,
            },
            RegExAndToken {
                regex: Regex::new("^\\*").unwrap(),
                token: Token::Asterisk,
            },
            RegExAndToken {
                regex: Regex::new("^\\[").unwrap(),
                token: Token::LeftBracket,
            },
            RegExAndToken {
                regex: Regex::new("^\\]").unwrap(),
                token: Token::RightBracket,
            },
            RegExAndToken {
                regex: Regex::new("^[0-9]+").unwrap(),
                token: Token::NumericConstant,
            },
            RegExAndToken {
                regex: Regex::new("^;").unwrap(),
                token: Token::Semicolon,
            },
           ],
        }
    }

    pub fn tokenize(&self, buf: &str) -> Vec<Token> {
        let mut slice_start: usize = 0;
        let mut v: Vec<Token> = Vec::new();

        'outer_loop: while slice_start < buf.len() {
            for reg_ex_and_token in &self.reg_ex_and_tokens {
                let next_token = reg_ex_and_token.regex.find(&buf[slice_start..]);
                match next_token {
                    Some(hit) => {
                        println!("Matched {}", reg_ex_and_token.regex);
                        println!("{} - {}", slice_start + hit.start(), slice_start + hit.end());
                        println!("{}", slice_start);
                        v.push(reg_ex_and_token.token);
                        slice_start += hit.end();
                        if let Some(whitespace_match) = Regex::new("^[ \\t\\r\\n]+").unwrap().find(&buf[slice_start..]) {
                            slice_start += whitespace_match.end();
                        }
                        // while buf.as_bytes()[slice_start] == b' ' || buf.as_bytes()[slice_start] == b'\n' || buf.as_bytes()[slice_start] == b'\r' {
                        //     slice_start += 1;
                        // }
                        continue 'outer_loop;
                    }
                    None => {
                        println!("Did not match: {}, {slice_start}", reg_ex_and_token.regex);
                        continue;
                    }
                }
            }
            panic!("Invalid token: \"{}\"", buf.as_bytes()[slice_start] as char);
        }
        v
    }
}
