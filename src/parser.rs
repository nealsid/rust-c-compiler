use crate::tokenizer::{Token, Tokenizer};

struct ASTNode {
    children : Vec<Box<ASTNode>>,
    token : Token
}

struct ProgramNode {
    function : FunctionNode,
}

struct FunctionNode {
    return_type : TypeNode,
    function_name : String
}

enum DataType {
    Integer,
}

struct TypeNode {
    data_type : DataType,
}


pub struct Parser {

}

impl Parser {
    fn parse_program(tokens : Vec<Token>) -> Result<ProgramNode, &'static str> {
        parse_function(tokens);
        
        ProgramNode::new()
    }

    fn parse_function(tokens : Vec<Token>) -> Result<FunctionNode, &'static str> {

    }

    fn parse_type(tokens : Vec<Token>) -> Result<TypeNode, &'static str_> {

    }
}