use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::tokenizer::Tokenizer;
use crate::parser::Parser;

pub mod parser;
pub mod tokenizer;
#[cfg(test)]
mod tokenizer_test;
fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);

    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut program_buffer = String::new();
    match file.read_to_string(&mut program_buffer) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("Read {} successfully", args[1]),
    }
    let t = Tokenizer::new();
    let tokens = t.tokenize(&program_buffer);
    dbg!(&tokens);
    let p = Parser { };
    let parse_result = p.parse_program(&tokens);
    dbg!(&parse_result);
}
