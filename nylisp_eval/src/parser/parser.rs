use crate::ast;
use crate::tokenizer;
use std::rc::Rc;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    pub fn parse_programs(self, tokens: Vec<String>) -> Vec<Result<ast::ast::NylispExpression, ast::ast::NylispError>> {
        let mut programs = Vec::new();
        let mut cur_tokens = tokens;
        loop {
            match self.parse_program(cur_tokens.clone()) {
                Ok((program, rest)) => {
                    programs.push(Ok(program));
                    if rest.is_empty() {
                        break;
                    }
                    cur_tokens = rest;
                }
                Err(err) => {
                    programs.push(Err(err));
                    break;
                }
            }
        }

        programs
    }

    pub fn parse_program(self, tokens: Vec<String>) -> Result<(ast::ast::NylispExpression, Vec<String>), ast::ast::NylispError> {
        let cur_token: String = tokens[0].clone();
        let rest_tokens: Vec<String> = tokens[1..].to_vec();

        // patturn patch the current token
        match cur_token.as_str() {
            tokenizer::tokenizer::LPAREN => {
                self.parse_list(rest_tokens)
            }
            tokenizer::tokenizer::QUOTE => {
                self.parse_quote(rest_tokens)
            }
            tokenizer::tokenizer::RPAREN => {
                Err(ast::ast::NylispError::Because("unexpected ')'".to_string()))
            }
            _ => {
                Ok((self.parse_atom(cur_token), rest_tokens))
            }
        }
    }

    fn parse_list(self, tokens: Vec<String>) -> Result<(ast::ast::NylispExpression, Vec<String>), ast::ast::NylispError> {
        let mut list_obj: Vec<ast::ast::NylispExpression> = Vec::new();
        let mut watching_tokens: Vec<String> = tokens;
        loop {
            let mut _cur_token: String = watching_tokens[0].clone();
            let mut _rest_tokens: Vec<String> = watching_tokens[1..].to_vec();
            if _cur_token.as_str() == tokenizer::tokenizer::RPAREN {
                return Ok((ast::ast::NylispExpression::List(list_obj), _rest_tokens));
            }

            let (cur_expr, rest_tokens) = self.parse_program(watching_tokens)?;
            list_obj.push(cur_expr);
            watching_tokens = rest_tokens;
        }
    }

    fn parse_atom(self, token: String) -> ast::ast::NylispExpression {
        let is_number: bool = token.parse::<f64>().is_ok();
        if is_number {
            return ast::ast::NylispExpression::Number(token.parse::<f64>().unwrap());
        }

        let is_boolearn: bool = token.as_str() == tokenizer::tokenizer::TRUE || token.as_str() == tokenizer::tokenizer::FALSE;
        if is_boolearn {
            return ast::ast::NylispExpression::Boolean(token.as_str() == tokenizer::tokenizer::TRUE);
        }

        ast::ast::NylispExpression::Symbol(token)
    }

    fn parse_quote(self, tokens: Vec<String>) -> Result<(ast::ast::NylispExpression, Vec<String>), ast::ast::NylispError> {
        let (cur_expr, parsed_rest_tokens) = self.parse_program(tokens)?;
        Ok((ast::ast::NylispExpression::Quote(Rc::new(cur_expr)), parsed_rest_tokens))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_nylisp_test1() {
        let input = "💖💔";
        let expected = ast::ast::NylispExpression::List(vec![]);

        let tokenizer_obj = tokenizer::tokenizer::Tokenizer::new(input.to_string());
        let tokens = tokenizer_obj.tokenize();
        let parser = Parser::new();
        let (result, _) = parser.parse_program(tokens).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn parser_nylisp_test2() {
        let input = "💖💖💖💔💔💔";
        let expected = ast::ast::NylispExpression::List(vec![
            ast::ast::NylispExpression::List(vec![
                ast::ast::NylispExpression::List(vec![]),
            ]),
        ]);

        let tokenizer_obj = tokenizer::tokenizer::Tokenizer::new(input.to_string());
        let tokens = tokenizer_obj.tokenize();
        let parser = Parser::new();
        let (result, _) = parser.parse_program(tokens).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn parser_nylisp_test3() {
        let input = "💖+ 1 2💔";
        let expected = ast::ast::NylispExpression::List(vec![
            ast::ast::NylispExpression::Symbol("+".to_string()),
            ast::ast::NylispExpression::Number(1.0),
            ast::ast::NylispExpression::Number(2.0),
        ]);

        let tokenizer_obj = tokenizer::tokenizer::Tokenizer::new(input.to_string());
        let tokens = tokenizer_obj.tokenize();
        let parser = Parser::new();
        let (result, _) = parser.parse_program(tokens).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn parser_nylisp_test4() {
        let input = "💖+ 1 💖+ 2 3💔💔";
        let expected = ast::ast::NylispExpression::List(vec![
            ast::ast::NylispExpression::Symbol("+".to_string()),
            ast::ast::NylispExpression::Number(1.0),
            ast::ast::NylispExpression::List(vec![
                ast::ast::NylispExpression::Symbol("+".to_string()),
                ast::ast::NylispExpression::Number(2.0),
                ast::ast::NylispExpression::Number(3.0),
            ]),
        ]);

        let tokenizer_obj = tokenizer::tokenizer::Tokenizer::new(input.to_string());
        let tokens = tokenizer_obj.tokenize();
        let parser = Parser::new();
        let (result, _) = parser.parse_program(tokens).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn parser_nylisp_quote() {
        let input = "💖☁️😪💖1 2 3💔💔";
        let expected = ast::ast::NylispExpression::List(vec![
            ast::ast::NylispExpression::Symbol("☁️".to_string()),
            ast::ast::NylispExpression::Quote(Rc::new(ast::ast::NylispExpression::List(vec![
                ast::ast::NylispExpression::Number(1.0),
                ast::ast::NylispExpression::Number(2.0),
                ast::ast::NylispExpression::Number(3.0),
            ]))),
        ]);

        let tokenizer_obj = tokenizer::tokenizer::Tokenizer::new(input.to_string());
        let tokens = tokenizer_obj.tokenize();
        let parser = Parser::new();
        let (result, _) = parser.parse_program(tokens).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn parser_multiple_list() {
        let input = "💖💔💖💔";
        let expected = ast::ast::NylispExpression::List(vec![
            ast::ast::NylispExpression::List(vec![]),
            ast::ast::NylispExpression::List(vec![]),
        ]);
        let tokenizer_obj = tokenizer::tokenizer::Tokenizer::new(input.to_string());
        let tokens = tokenizer_obj.tokenize();
        let parser = Parser::new();
        let res = parser.parse_programs(tokens);
        assert_eq!(res.len(), 2);
    }
}
