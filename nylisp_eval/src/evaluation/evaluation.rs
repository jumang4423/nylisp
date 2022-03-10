use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use crate::ast::*;
use crate::tokenizer;
use crate::parser;

#[derive(Debug, Clone)]
pub struct Evaluator {
    program: Vec<ast::NylispExpression>,
}

impl Iterator for Evaluator {
    type Item = ast::NylispExpression;
    fn next(&mut self) -> Option<Self::Item> {
        self.program.pop()
    }
}

impl Evaluator {
    pub fn new(program: Vec<ast::NylispExpression>) -> Evaluator {
        Evaluator {
            program
        }
    }

    pub fn eval_programs(&mut self, env: &mut ast::Environment) -> Vec<Result<ast::NylispExpression, ast::NylispError>> {
        let mut result: Vec<Result<ast::NylispExpression, ast::NylispError>> = Vec::new();
        for expr in self.program.iter() {
            result.push(self.evaluate(expr, env));
        }

        result
    }

    fn evaluate(&self, exp: &ast::NylispExpression, env: &'_ mut ast::Environment) -> Result<ast::NylispExpression, ast::NylispError> {
        match exp {
            ast::NylispExpression::Quote(q) => Ok((**q).clone()),
            ast::NylispExpression::Boolean(b) => Ok(ast::NylispExpression::Boolean(*b)),
            ast::NylispExpression::Number(n) => Ok(ast::NylispExpression::Number(*n)),
            ast::NylispExpression::String(s) => Ok(ast::NylispExpression::String(s.clone())),
            ast::NylispExpression::Symbol(s) => {
                // get from env
                match ast::get(s.as_str(), env) {
                    Some(e) => Ok(e),
                    None => Err(ast::NylispError::Because(format!("symbol {} not found in environment", s)))
                }
            }
            ast::NylispExpression::Function(f) => Ok(ast::NylispExpression::Function(*f)),
            ast::NylispExpression::List(l) => {
                let first: ast::NylispExpression = match l.first() {
                    Some(e) => e.clone(),
                    None => return Ok(ast::NylispExpression::Boolean(false))
                };
                let rest: Vec<ast::NylispExpression> = l[1..].iter().map(|e| e.clone()).collect::<Vec<ast::NylispExpression>>();

                // TODO: dirty as fuck but works
                match self.wait_a_minute_is_this_a_special_form(first.clone(), rest.clone(), env) {
                    Ok(e) => {
                        match e {
                            Some(e) => Ok(e),
                            None => {
                                match self.evaluate(&first, env) {
                                    Ok(e) => {
                                        match e {
                                            ast::NylispExpression::Function(f) => {
                                                let mut evaluated_args: Vec<ast::NylispExpression> = Vec::new();
                                                for arg in rest {
                                                    evaluated_args.push(self.evaluate(&arg, env)?);
                                                }
                                                f(evaluated_args)
                                            }
                                            ast::NylispExpression::Closure { args, body: cl_body } => {
                                                self.evaluate(&*cl_body, &mut self.new_closure_env(args, rest, env).unwrap())
                                            }
                                            _ => Err(ast::NylispError::Because(format!("not a function: {:?}", first)))
                                        }
                                    }
                                    Err(e) => Err(e)
                                }
                            }
                        }
                    }
                    Err(e) => Err(e)
                }
            }
            _ => Err(ast::NylispError::Because(format!("unsupported expression type: {:?}", exp)))
        }
    }

    fn wait_a_minute_is_this_a_special_form(&self, exp: ast::NylispExpression, args: Vec<ast::NylispExpression>, env: &mut ast::Environment) -> Result<Option<ast::NylispExpression>, ast::NylispError> {
        match exp {
            ast::NylispExpression::Symbol(s) => {
                match s.as_str() {
                    tokenizer::tokenizer::IF => {
                        if args.len() != 3 {
                            return Err(ast::NylispError::Because(format!("🐶 requires 3 arguments, got {}", args.len())));
                        }
                        let condition: ast::NylispExpression = self.evaluate(&args[0], env)?;
                        let then_branch: ast::NylispExpression = self.evaluate(&args[1], env)?;
                        let else_branch: ast::NylispExpression = self.evaluate(&args[2], env)?;
                        match condition {
                            ast::NylispExpression::Boolean(b) => {
                                if b {
                                    return Ok(Some(then_branch));
                                } else {
                                    return Ok(Some(else_branch));
                                }
                            }
                            _ => return Err(ast::NylispError::Because(format!("🐶 requires a boolean condition, got {:?}", condition)))
                        }
                    }
                    tokenizer::tokenizer::VAR => {
                        if args.len() != 2 {
                            return Err(ast::NylispError::Because(format!("🌷 requires 2 arguments, got {}", args.len())));
                        }
                        let value: ast::NylispExpression = self.evaluate(&args[1], env)?;
                        if let ast::NylispExpression::Symbol(s) = args[0].clone() {
                            env.data.insert(s.clone(), value.clone());
                            return Ok(Some(ast::NylispExpression::List(vec![
                                ast::NylispExpression::Symbol(s.clone()),
                                value,
                                ast::NylispExpression::Boolean(true),
                            ])));
                        } else {
                            return Err(ast::NylispError::Because(format!("🌷 requires a symbol as first argument, got {:?}", args[0])));
                        }
                    }
                    tokenizer::tokenizer::CLOSURE => {
                        // closure
                        if args.len() != 2 {
                            return Err(ast::NylispError::Because(format!("🏨 requires 2 arguments, got {}", args.len())));
                        }

                        return Ok(
                            Some(
                                ast::NylispExpression::Closure {
                                    args: Rc::new(args[0].clone()),
                                    body: Rc::new(args[1].clone()),
                                }
                            )
                        );
                    }
                    tokenizer::tokenizer::SCOPED_LET => {
                        // closure
                        if args.len() != 2 {
                            return Err(ast::NylispError::Because(format!("🍙 requires 2 arguments, got {}", args.len())));
                        }

                        match self.evaluate(&args[1], &mut self.new_scoped_let_env(Rc::new(args[0].clone()), env).unwrap()) {
                            Ok(evaluated_exp) => {
                                return Ok(Some(evaluated_exp));
                            }
                            Err(e) => return Err(e)
                        }
                    }
                    _ => return Ok(None)
                }
            }
            _ => return Ok(None)
        }
    }

    fn new_closure_env<'a>(&self, param: Rc<ast::NylispExpression>, arg: Vec<ast::NylispExpression>, env: &'a mut ast::Environment) -> Result<ast::Environment<'a>, ast::NylispError> {
        let mut param_strings = self.strs_from_list_of_symbols((*param).clone())?;
        if param_strings.len() != arg.len() {
            return Err(ast::NylispError::Because(format!("🏨 requires the same number of arguments as parameters, got {} and {}", param_strings.len(), arg.len())));
        }
        let evaled_args = arg.iter().map(|e| self.evaluate(&e, env)).collect::<Result<Vec<ast::NylispExpression>, ast::NylispError>>()?;
        let mut data: std::collections::HashMap<String, ast::NylispExpression> = std::collections::HashMap::new();
        for (k, v) in param_strings.iter().zip(evaled_args.iter()) {
            data.insert(k.clone(), v.clone());
        }
        Ok(ast::Environment {
            _virtual: Some(env),
            data: data,
        })
    }

    fn new_scoped_let_env<'a>(&self, variables: Rc<ast::NylispExpression>, env: &'a mut ast::Environment) -> Result<ast::Environment<'a>, ast::NylispError> {
        // variables is something like ((a 1) (b 2))
        let mut data: std::collections::HashMap<String, ast::NylispExpression> = std::collections::HashMap::new();
        // check variables is a list
        if let ast::NylispExpression::List(variable_list) = (*variables).clone() {
            for var in variable_list {
                // check var is a list
                if let ast::NylispExpression::List(var_list) = var {
                    if var_list.len() != 2 {
                        return Err(ast::NylispError::Because(format!("🍙 requires a list of 2 elements, got {}", var_list.len())));
                    }
                    let var_name = self.str_from_symbol(var_list[0].clone())?;
                    let var_value = self.evaluate(&var_list[1], env)?;
                    data.insert(var_name, var_value);
                } else {
                    return Err(ast::NylispError::Because(format!("🍙 requires a list of 2 elements, got {:?}", var)));
                }
            }
        } else {
            return Err(ast::NylispError::Because(format!("🍙 first element should be a list, but got {:?}", variables)));
        }

        Ok(ast::Environment {
            _virtual: Some(env),
            data,
        })
    }

    fn strs_from_list_of_symbols(&self, list: ast::NylispExpression) -> Result<Vec<String>, ast::NylispError> {
        match list {
            ast::NylispExpression::List(list) => {
                let mut strings = vec![];
                for item in &list {
                    match item {
                        ast::NylispExpression::Symbol(s) => {
                            strings.push(s.clone());
                        }
                        _ => return Err(ast::NylispError::Because(format!("expected symbol, but got {:?}", list)))
                    }
                }
                return Ok(strings);
            }
            _ => return Err(ast::NylispError::Because(format!("expected list, but got {:?}", list)))
        }
    }

    fn str_from_symbol(&self, symbol: ast::NylispExpression) -> Result<String, ast::NylispError> {
        match symbol {
            ast::NylispExpression::Symbol(s) => Ok(s.clone()),
            _ => return Err(ast::NylispError::Because(format!("expected symbol, but got {:?}", symbol)))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::environment;
    use super::*;

    fn input_and_go(input: &str) -> Vec<Result<ast::NylispExpression, ast::NylispError>> {
        let mut tokenizer_obj = tokenizer::tokenizer::Tokenizer::new(input.to_string());
        let tokens = tokenizer_obj.tokenize();
        let mut parser = parser::parser::Parser::new();
        let (result, _) = parser.parse_program(tokens).unwrap();
        let mut evaluator = Evaluator::new(vec![result]);
        let result = evaluator.eval_programs(&mut ast::Environment::from(environment::environment::builtin_env()));

        result
    }

    // +
    #[test]
    fn eval_nylisp_test_plus() {
        let input = "💖+ 1 2💔";
        let expected: Result<ast::NylispExpression, ast::NylispError> = Ok(ast::NylispExpression::Number(3.0));
        let got = input_and_go(input);
        assert_eq!(got[0], expected);
    }

    // -
    #[test]
    fn eval_nylisp_test_minus() {
        let input = "💖- 1 2💔";
        let expected: Result<ast::NylispExpression, ast::NylispError> = Ok(ast::NylispExpression::Number(-1.0));
        let got = input_and_go(input);
        assert_eq!(got[0], expected);
    }

    // <>
    #[test]
    fn eval_nylisp_test_bigger_or_smaller() {
        let input = "💖> 1 2 3 4💔";
        let expected = ast::NylispExpression::Boolean(false);
        let got = input_and_go(input);
        assert_eq!(got[0], Ok(expected));

        let input = "💖< 1 2 3 4💔";
        let expected = ast::NylispExpression::Boolean(true);
        let got = input_and_go(input);
        assert_eq!(got[0], Ok(expected));
    }

    // car and quote
    #[test]
    fn eval_nylisp_test_car_quote() {
        let input = "💖🚗😪💖👎 👍 👍💔💔";
        let expected = ast::NylispExpression::Boolean(false);
        let got = input_and_go(input);
        assert_eq!(got[0], Ok(expected));
    }

    // not and car 🚗
    #[test]
    fn eval_nylisp_test_not_car() {
        let input = "💖🚗💖❌ 💖= 1 2💔💖= 1 3💔💖= 1 4💔💔💔";
        let expected = ast::NylispExpression::Boolean(true);
        let got = input_and_go(input);
        assert_eq!(got[0], Ok(expected));
    }

    // not and cdr 💭
    #[test]
    fn eval_nylisp_test_not_cdr() {
        let input = "💖💭💖❌ 💖= 1 2💔💖= 1 3💔💖= 1 4💔💔💔";
        let expected = ast::NylispExpression::List(vec![ast::NylispExpression::Boolean(true), ast::NylispExpression::Boolean(true)]);
        let got = input_and_go(input);
        assert_eq!(got[0], Ok(expected));
    }

    // quote and cdr 💭
    #[test]
    fn eval_nylisp_test_cdr() {
        let input = "💖💭😪💖1 2💔💔";
        let expected = ast::NylispExpression::List(vec![ast::NylispExpression::Number(2.0)]);
        let got = input_and_go(input);
        assert_eq!(got[0], Ok(expected));
    }

    // err quotation 💭
    #[test]
    fn eval_nylisp_err_quote_it() {
        let input = "💖💩 1 2 3💔";
        let expected: String = "symbol 💩 not found in environment".to_string();
        let got = input_and_go(input);
        assert_eq!(got[0].as_ref().unwrap_err().to_string(), expected);
    }

    // global variable var access
    #[test]
    fn eval_nylisp_insert_dat() {
        let input = "💖🌹 hoge 😪💖1 2 3💔💔";
        let expected = ast::NylispExpression::List(vec![
            ast::NylispExpression::Symbol("hoge".to_string()),
            ast::NylispExpression::List(vec![
                ast::NylispExpression::Number(1.0),
                ast::NylispExpression::Number(2.0),
                ast::NylispExpression::Number(3.0)])
            , ast::NylispExpression::Boolean(true)]);
        let got = input_and_go(input);
        assert_eq!(got[0], Ok(expected));
    }

    // if statement
    #[test]
    fn eval_nylisp_if_statement() {
        let input = "💖🐶 💖🚗😪💖👎 👍 👍💔💔 😪ok 😪unexpected💔";
        let expected = ast::NylispExpression::Symbol("unexpected".to_string());
        let got = input_and_go(input);
        assert_eq!(got[0], Ok(expected));
    }

    // lambda
    #[test]
    fn eval_nylisp_lambda() {
        let input = "💖💖🐷💖 x 💔 💖🚗 x💔💔😪💖ok no💔💔";
        let expected = ast::NylispExpression::Symbol("ok".to_string());
        let got = input_and_go(input);
        assert_eq!(got[0], Ok(expected));
    }

    // scoped let
    #[test]
    fn eval_nylisp_scoped_let() {
        let input = "💖🍙 💖💖x 2💔💖y 2💔💔 💖+ x y💔💔";
        let expected = ast::NylispExpression::Number(4.0);
        let got = input_and_go(input);
        assert_eq!(got[0], Ok(expected));
    }

    // random 🎨
    #[test]
    fn eval_nylisp_random() {
        let input = "💖🎨 123456789💔";
        let got = input_and_go(input);
        if let ast::NylispExpression::Number(_) = got[0].as_ref().unwrap() {
            assert!(true);
        } else {
            assert!(false);
        }
    }
}