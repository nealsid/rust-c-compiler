use crate::tokenizer::Token;

enum NodeType {
    ProgramNode,
    FunctionNode { return_type: DataType, function_name: String },
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
    pub fn parse_program(&self, tokens : Vec<Token>) -> Result<ASTNode, &'static str> {
        let mut program_node = ASTNode {
            children: vec![],
            token : None,
            node_type : NodeType::ProgramNode
        };
        let mut token_counter = 0;
        self.parse_function(tokens, token_counter);
        
        Ok(program_node)
    }

    fn parse_function(&self, tokens : Vec<Token>, token_counter : u32) -> Result<ASTNode, &'static str> {
        
        self.parse_data_type(tokens, token_counter);
//        self.parse_function_name(tokens, token_counter);
        let mut function_node = ASTNode {
            children: vec![],
            token: None,
            node_type : NodeType::FunctionNode { return_type: DataType::Integer, function_name: String::from("main") }
        };

        Ok(function_node)
    }

    fn parse_data_type(&self, tokens : Vec<Token>, token_counter : u32) -> Result<DataType, &'static str> {
        let data_type_token = &tokens[0];
        if let Token::Keyword{ keyword: s } = data_type_token {
            println!("parsed type {}", s);
        } else {
            println!("parse error");
        }
        Ok(DataType::Integer)
    }
}