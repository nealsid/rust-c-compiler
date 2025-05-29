use crate::tokenizer::{Token, TokenInfo};

enum NodeType {
    ProgramNode,
    FunctionNode { return_type: Result<DataType, &'static str>, function_name: String },
}

pub struct ASTNode {
    children : Vec<ASTNode>,
    token : Option<Token>,
    node_type : NodeType
}

enum DataType {
    Integer,
}

pub struct Parser {

}

impl Parser {
    pub fn parse_program(&self, tokens : Vec<TokenInfo>) -> Result<ASTNode, &'static str> {
        let mut program_node = ASTNode {
            children: vec![],
            token : None,
            node_type : NodeType::ProgramNode
        };
        let mut token_counter = 0;
        self.parse_function(tokens, token_counter);
        
        Ok(program_node)
    }

    fn parse_function(&self, tokens : Vec<TokenInfo>, token_counter : usize) -> Result<ASTNode, &'static str> {
        
        let data_type_result = self.parse_data_type(tokens, token_counter);
        let function_name_result = self.parse_function_name(tokens, token_counter);
//        self.parse_function_name(tokens, token_counter);
        let function_node = ASTNode {
            children: vec![],
            token: None,
            node_type : NodeType::FunctionNode { return_type: data_type_result, function_name: String::from("main") }
        };

        Ok(function_node)
    }

    fn parse_function_name(&self, tokens: Vec<TokenInfo>, token_counter: usize) -> Result<String, &'static str> {

    }
    fn parse_data_type(&self, tokens : Vec<TokenInfo>, token_counter : usize) -> Result<DataType, &'static str> {
        let data_type_token = &tokens[token_counter];
        if let Token::Keyword{ keyword: s } = &data_type_token.token {
            match s.as_str() {
                "int" => Ok(DataType::Integer),
                _ => Err("Parse error")
            }
        } else {
            Err("parse error")
        }
    }
}