use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::env::args;
use crate::ast;
use crate::evaluation;
use rand::Rng;

pub fn builtin_env<'a>() -> ast::ast::Environment<'a> {
    let mut data: HashMap<String, ast::ast::NylispExpression> = HashMap::new();

    // +
    data.insert("+".to_string(), ast::ast::NylispExpression::Function(
        |args: Vec<ast::ast::NylispExpression>| -> Result<ast::ast::NylispExpression, ast::ast::NylispError> {
            let sum = parse_list_of_floats(&args)?.iter().fold(0.0, |sum, a| sum + a);

            Ok(ast::ast::NylispExpression::Number(sum))
        }
    ));

    // -
    data.insert("-".to_string(), ast::ast::NylispExpression::Function(
        |args: Vec<ast::ast::NylispExpression>| -> Result<ast::ast::NylispExpression, ast::ast::NylispError> {
            let args = parse_list_of_floats(&args)?;
            // pop the first element
            let mut result = *args.first().unwrap();
            let rest_args = args.iter().skip(1);
            for arg in rest_args {
                result -= arg;
            }

            Ok(ast::ast::NylispExpression::Number(result))
        }
    ));

    // *
    data.insert("*".to_string(), ast::ast::NylispExpression::Function(
        |args: Vec<ast::ast::NylispExpression>| -> Result<ast::ast::NylispExpression, ast::ast::NylispError> {
            let args = parse_list_of_floats(&args)?;
            // pop the first element
            let mut result = *args.first().unwrap();
            let rest_args = args.iter().skip(1);
            for arg in rest_args {
                result *= arg;
            }

            Ok(ast::ast::NylispExpression::Number(result))
        }
    ));

    // /
    data.insert("/".to_string(), ast::ast::NylispExpression::Function(
        |args: Vec<ast::ast::NylispExpression>| -> Result<ast::ast::NylispExpression, ast::ast::NylispError> {
            let args = parse_list_of_floats(&args)?;
            // pop the first element
            let mut result = *args.first().unwrap();
            let rest_args = args.iter().skip(1);
            for arg in rest_args {
                result /= arg;
            }
            Ok(ast::ast::NylispExpression::Number(result))
        }
    ));

    // %
    data.insert("%".to_string(), ast::ast::NylispExpression::Function(
        |args: Vec<ast::ast::NylispExpression>| -> Result<ast::ast::NylispExpression, ast::ast::NylispError> {
            let args = parse_list_of_floats(&args)?;
            // pop the first element
            let mut result = *args.first().unwrap();
            let rest_args = args.iter().skip(1);
            for arg in rest_args {
                result %= arg;
            }

            Ok(ast::ast::NylispExpression::Number(result))
        }
    ));

    // =
    data.insert("=".to_string(), ast::ast::NylispExpression::Function(
        |args: Vec<ast::ast::NylispExpression>| -> Result<ast::ast::NylispExpression, ast::ast::NylispError> {
            let first = args.first().unwrap();
            let rest = args.iter().skip(1);
            for arg in rest {
                if first != arg {
                    return Ok(ast::ast::NylispExpression::Boolean(false));
                }
            }

            Ok(ast::ast::NylispExpression::Boolean(true))
        }
    ));

    // <
    data.insert("<".to_string(), ast::ast::NylispExpression::Function(
        |args: Vec<ast::ast::NylispExpression>| -> Result<ast::ast::NylispExpression, ast::ast::NylispError> {
            let args = parse_list_of_floats(&args)?;
            // pop the first element
            let mut result = *args.first().unwrap();
            let rest_args = args.iter().skip(1);
            for arg in rest_args {
                if result >= *arg {
                    return Ok(ast::ast::NylispExpression::Boolean(false));
                }
                result = *arg;
            }

            Ok(ast::ast::NylispExpression::Boolean(true))
        }
    ));

    // >
    data.insert(">".to_string(), ast::ast::NylispExpression::Function(
        |args: Vec<ast::ast::NylispExpression>| -> Result<ast::ast::NylispExpression, ast::ast::NylispError> {
            let args = parse_list_of_floats(&args)?;
            // pop the first element
            let mut result = *args.first().unwrap();
            let rest_args = args.iter().skip(1);
            for arg in rest_args {
                if result <= *arg {
                    return Ok(ast::ast::NylispExpression::Boolean(false));
                }
                result = *arg;
            }

            Ok(ast::ast::NylispExpression::Boolean(true))
        }
    ));

    // and
    data.insert("😎".to_string(), ast::ast::NylispExpression::Function(
        |args: Vec<ast::ast::NylispExpression>| -> Result<ast::ast::NylispExpression, ast::ast::NylispError> {
            let mut args = parse_list_of_bools(&args)?;
            // pop the first element
            let result = args.pop().unwrap();
            for a in args {
                if !a {
                    return Ok(ast::ast::NylispExpression::Boolean(false));
                }
            }

            Ok(ast::ast::NylispExpression::Boolean(result))
        }
    ));

    // or
    data.insert("😕".to_string(), ast::ast::NylispExpression::Function(
        |args: Vec<ast::ast::NylispExpression>| -> Result<ast::ast::NylispExpression, ast::ast::NylispError> {
            let mut args = parse_list_of_bools(&args)?;
            // pop the first element
            let result = args.pop().unwrap();
            for a in args {
                if a {
                    return Ok(ast::ast::NylispExpression::Boolean(true));
                }
            }

            Ok(ast::ast::NylispExpression::Boolean(result))
        }
    ));

    // not
    data.insert("❌".to_string(), ast::ast::NylispExpression::Function(
        |args: Vec<ast::ast::NylispExpression>| -> Result<ast::ast::NylispExpression, ast::ast::NylispError> {
            let args = parse_list_of_bools(&args)?;
            let mut result: Vec<ast::ast::NylispExpression> = vec![];
            for a in args {
                result.push(ast::ast::NylispExpression::Boolean(!a));
            }

            Ok(ast::ast::NylispExpression::List(result))
        }
    ));

    // car
    data.insert("🚗".to_string(), ast::ast::NylispExpression::Function(
        |args: Vec<ast::ast::NylispExpression>| -> Result<ast::ast::NylispExpression, ast::ast::NylispError> {
            // validate
            if args.len() != 1 {
                return Err(ast::ast::NylispError::Because(
                    "🚗 requires exactly one argument".to_string(),
                ));
            }
            // get first element of list
            let cons = parse_single_list(&args[0])?;
            Ok(cons.first().unwrap().clone())
        }
    ));

    // cdr
    data.insert("💭".to_string(), ast::ast::NylispExpression::Function(
        |args: Vec<ast::ast::NylispExpression>| -> Result<ast::ast::NylispExpression, ast::ast::NylispError> {
            let cons = parse_single_list(&args[0])?;
            Ok(ast::ast::NylispExpression::List(cons[1..].to_vec()))
        }
    ));

    // random
    data.insert("🎨".to_string(), ast::ast::NylispExpression::Function(
        |args: Vec<ast::ast::NylispExpression>| -> Result<ast::ast::NylispExpression, ast::ast::NylispError> {
            if let ast::ast::NylispExpression::Number(n) = &args[0] {
                let index = rand::thread_rng().gen_range(0, *n as usize);
                Ok(ast::ast::NylispExpression::Number(index as f64))
            } else {
                Err(ast::ast::NylispError::Because(
                    "🎨 requires a number as first argument".to_string(),
                ))
            }
        }
    ));

    // return data
    ast::ast::Environment {
        data,
        _virtual: None,
    }
}

fn parse_list_of_floats(args: &[ast::ast::NylispExpression]) -> Result<Vec<f64>, ast::ast::NylispError> {
    args
        .iter()
        .map(|x| parse_single_float(x))
        .collect()
}

fn parse_single_float(exp: &ast::ast::NylispExpression) -> Result<f64, ast::ast::NylispError> {
    match exp {
        ast::ast::NylispExpression::Number(num) => Ok(*num),
        _ => Err(ast::ast::NylispError::Because("expected a number".to_string())),
    }
}

fn parse_list_of_bools(args: &[ast::ast::NylispExpression]) -> Result<Vec<bool>, ast::ast::NylispError> {
    args
        .iter()
        .map(|x| parse_single_bool(x))
        .collect()
}

fn parse_single_bool(exp: &ast::ast::NylispExpression) -> Result<bool, ast::ast::NylispError> {
    match exp {
        ast::ast::NylispExpression::Boolean(b) => Ok(*b),
        _ => Err(ast::ast::NylispError::Because("expected a boolean".to_string())),
    }
}

fn parse_single_list(exp: &ast::ast::NylispExpression) -> Result<Vec<ast::ast::NylispExpression>, ast::ast::NylispError> {
    match exp {
        ast::ast::NylispExpression::List(list) => Ok(list.clone()),
        _ => Err(ast::ast::NylispError::Because("expected a list".to_string())),
    }
}