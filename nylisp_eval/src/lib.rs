extern crate core;

// nylisp interpreter library
mod parser;
mod tokenizer;
pub mod ast;
pub mod environment;
mod evaluation;

// export ast, environment to lib.rs
pub use ast::*;
pub use environment::*;

// tokenize given code then return a list of tokens
pub fn tokenize_nylisp(plaintext: String) -> Vec<String> {
    let tokenizer_obj = tokenizer::tokenizer::Tokenizer::new(plaintext);
    tokenizer_obj.tokenize()
}

// parse given tokens into an AST
pub fn parse_nylisps(tokens: Vec<String>) -> Vec<Result<ast::ast::NylispExpression, ast::ast::NylispError>> {
    let mut parser = parser::parser::Parser::new();
    parser.parse_programs(tokens)
}

// evaluate given AST
pub fn evaluate_nylisp(ast: Vec<ast::ast::NylispExpression>) -> Vec<Result<ast::ast::NylispExpression, ast::ast::NylispError>> {
    let mut env = environment::environment::builtin_env();
    let mut evaluator = evaluation::evaluation::Evaluator::new(ast);
    evaluator.eval_programs(&mut env)
}