#[cfg(test)]
mod tokenizer_test {
    use crate::tokenizer::{Token, Tokenizer};

    macro_rules! single_token_tests {
        ($($name:ident: ($expected_token:expr, $input_str:literal),)*) => {
        $(
            #[test]
            fn $name() {
                let tokenizer = Tokenizer::new();
                let tokens = tokenizer.tokenize($input_str);
                assert_eq!(1, tokens.len());
                assert_eq!($expected_token, tokens[0].token);

                let tokens = tokenizer.tokenize(format!("\t{}\t", $input_str).as_str());
                assert_eq!(1, tokens.len());
                assert_eq!($expected_token, tokens[0].token);
            }
        )*
        }
    }

    single_token_tests! {
        lparen: (Token::LeftParen, "("),
        rparen: (Token::RightParen, ")"),
        keyword: (Token::Keyword { keyword: String::from("main") }, "main"),
        lbrace: (Token::LeftBrace, "{"),
        rbrace: (Token::RightBrace, "}"),
        comma: (Token::Comma, ","),
        asterisk: (Token::Asterisk, "*"),
        lbracket: (Token::LeftBracket, "["),
        rbracket: (Token::RightBracket, "]"),
        numeric_constant: (Token::NumericConstant, "9"),
        semicolon: (Token::Semicolon, ";"),
    }
}
