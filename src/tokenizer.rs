use regex::Regex;

macro_rules! regex {
    ($str:literal) => { Regex::new($str).unwrap() };
}

struct RegExAndToken {
    regex: Regex,
    token: Token,
}

#[derive(Debug)]
pub struct TokenInfo {
    token : Token,
    line_number : u32,
    column_number : u32
}

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum Token {
    LeftParen = 0,
    RightParen = 1,
    Keyword { keyword: String } = 2,
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
                    regex: regex![r"^\("],
                    token: Token::LeftParen,
                },
                RegExAndToken {
                    regex: regex![r"^\)"],
                    token: Token::RightParen,
                },
                RegExAndToken {
                    regex: regex![r"^\{"],
                    token: Token::LeftBrace,
                },
                RegExAndToken {
                    regex: regex![r"^\}"],
                    token: Token::RightBrace,
                },
                RegExAndToken {
                    regex: regex![r"^[A-Za-z_]+"],
                    token: Token::Keyword {
                        keyword: String::from(""),
                    },
                },
                RegExAndToken {
                    regex: regex![r"^,"],
                    token: Token::Comma,
                },
                RegExAndToken {
                    regex: regex![r"^\*"],
                    token: Token::Asterisk,
                },
                RegExAndToken {
                    regex: regex![r"^\["],
                    token: Token::LeftBracket,
                },
                RegExAndToken {
                    regex: regex![r"^\]"],
                    token: Token::RightBracket,
                },
                RegExAndToken {
                    regex: regex![r"^[0-9]+"],
                    token: Token::NumericConstant,
                },
                RegExAndToken {
                    regex: regex![r"^;"],
                    token: Token::Semicolon,
                },
            ],
        }
    }

    pub fn tokenize(&self, buf: &str) -> Vec<TokenInfo> {
        let mut slice_start: usize = 0;
        let mut current_line_number: u32 = 1;
        let mut v: Vec<TokenInfo> = Vec::new();
        let mut matched_newline = false;

        'outer_loop: while slice_start < buf.len() {
            matched_newline = false;
            // if match whitespace, advance slice_start
            if let Some(whitespace_match) = regex![r"^[ \t]+"]
                .find(&buf[slice_start..])
            {
                slice_start += whitespace_match.end();
            }
            // while match newline, increment line number and continue
            while let Some(newline_match) = regex![r"^(\r?\n)|^(\n\r?)"]
                .find(&buf[slice_start..])
            {
                slice_start += newline_match.end();
                current_line_number += 1;
                matched_newline = true;
            }

            // If we matched a newline, start over to consume whitespace again
            if matched_newline {
                continue;
            }

            for reg_ex_and_token in &self.reg_ex_and_tokens {
                let next_token = reg_ex_and_token.regex.find(&buf[slice_start..]);
                match next_token {
                    Some(hit) => {
                        // The hit returned by regex::find is relative to the slice
                        // we passed into it, so we need to turn it into positions
                        // that are relative to the beginning of buf.
                        let match_start = slice_start + hit.start();
                        let match_end = slice_start + hit.end();
                        println!("Matched {}: \"{}\"", reg_ex_and_token.regex, &buf[match_start..match_end]);
                        println!("{} - {}", match_start, match_end);
                        println!("{}", slice_start);
                        let t: Token;
                        t = match reg_ex_and_token.token {
                            Token::Keyword { keyword: _ } => Token::Keyword {
                                keyword: String::from(&buf[match_start..match_end]),
                            },
                            _ => reg_ex_and_token.token.clone(),
                        };
                        let token_info = TokenInfo {
                            token: t,
                            line_number: current_line_number,
                            column_number: 0
                        };
                        v.push(token_info);
                        slice_start += hit.end();
                        continue 'outer_loop;
                    }
                    None => {
                        continue;
                    }
                }
            }
            panic!("Invalid token: \"{}\"", buf.as_bytes()[slice_start] as char);
        }
        v
    }
}
