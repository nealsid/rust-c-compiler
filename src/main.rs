use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::tokenizer::Tokenizer;
use crate::parser::Parser;

pub mod parser;
pub mod tokenizer;

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
        Ok(_) => println!("Read {} successfully", args[1]),
    }
    let t = Tokenizer::new();
    let tokens = t.tokenize(&s);
    let p = Parser { };
    p.parse_program(tokens);
}
