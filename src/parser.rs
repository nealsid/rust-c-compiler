use crate::tokenizer::{Token, TokenInfo};

#[derive(Debug)]
enum NodeType {
    ProgramNode,
    FunctionNode {
        return_type: Result<DataType, &'static str>,
        function_name: Result<String, &'static str>,
    },
}

#[derive(Debug)]
pub struct ASTNode {
    children: Vec<Result<ASTNode, &'static str>>,
    token: Option<Token>,
    node_type: NodeType,
}

#[derive(Debug)]
enum DataType {
    Integer,
}

pub struct Parser {}

impl Parser {
    pub fn parse_program(&self, tokens: &Vec<TokenInfo>) -> Result<ASTNode, &'static str> {
        let mut program_node = ASTNode {
            children: vec![],
            token: None,
            node_type: NodeType::ProgramNode,
        };
        let mut token_counter: usize = 0;
        program_node
            .children
            .push(self.parse_function(tokens, &mut token_counter));

        Ok(program_node)
    }

    fn parse_function(
        &self,
        tokens: &Vec<TokenInfo>,
        token_counter: &mut usize,
    ) -> Result<ASTNode, &'static str> {
        let data_type_result = self.parse_data_type(tokens, *token_counter);
        *token_counter += 1;
        let function_name_result = self.parse_function_name(tokens, *token_counter);
        *token_counter += 1;
        if !self.parse_left_paren(tokens, *token_counter) {
            return Err("Missing left paren");
        }

        *token_counter += 1;
        let function_node = ASTNode {
            children: vec![],
            token: None,
            node_type: NodeType::FunctionNode {
                return_type: data_type_result,
                function_name: function_name_result,
            },
        };

        Ok(function_node)
    }

    fn parse_data_type(
        &self,
        tokens: &Vec<TokenInfo>,
        token_index: usize,
    ) -> Result<DataType, &'static str> {
        let data_type_token = &tokens[token_index];
        if let Token::Keyword { keyword: s } = &data_type_token.token {
            match s.as_str() {
                "int" => Ok(DataType::Integer),
                _ => Err("Parse error"),
            }
        } else {
            Err("parse error")
        }
    }

    fn parse_function_name(
        &self,
        tokens: &Vec<TokenInfo>,
        token_index: usize,
    ) -> Result<String, &'static str> {
        let function_name_token = &tokens[token_index];
        if let Token::Keyword { keyword: s } = &function_name_token.token {
            Ok(s.to_string())
        } else {
            Err("Function name parse error")
        }
    }

    fn parse_left_brace(&self, tokens: &Vec<TokenInfo>, token_index: usize) -> bool {
        let next_token = &tokens[token_index];
        match next_token.token {
            Token::LeftBrace => true,
            _ => false,
        }
    }

    fn parse_left_paren(&self, tokens: &Vec<TokenInfo>, token_index: usize) -> bool {
        let next_token = &tokens[token_index];
        match next_token.token {
            Token::LeftParen => true,
            _ => false,
        }
    }
}
