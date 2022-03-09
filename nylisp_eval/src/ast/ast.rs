use std::collections::HashMap;
use std::path::Display;
use std::rc::Rc;

// exp
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum NylispExpression {
    Quote(Rc<NylispExpression>),
    Symbol(String),
    Number(f64),
    Boolean(bool),
    String(String),
    List(Vec<NylispExpression>),
    Function(fn(Vec<NylispExpression>) -> Result<NylispExpression, NylispError>),
    Closure {
        args: Rc<NylispExpression>,
        body: Rc<NylispExpression>,
    },
    ScopedLet {
        variables: Rc<NylispExpression>,
        body: Rc<NylispExpression>,
    },
}

// environment
#[derive(Clone, Debug, PartialEq)]
pub struct Environment<'a> {
    pub(crate) data: HashMap<String, NylispExpression>,
    pub(crate) _virtual: Option<&'a Environment<'a>>
}

pub fn get(key: &str, env: &Environment) -> Option<NylispExpression> {
    match env.data.get(key) {
        Some(exp) => Some(exp.clone()),
        None => {
            match &env._virtual {
                Some(virtual_env) => get(key, virtual_env),
                None => None,
            }
        }
    }
}

// internal errorObject
#[derive(Clone, Debug, PartialEq)]
pub enum NylispError {
    Because(String),
}

impl std::fmt::Display for NylispError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            NylispError::Because(ref s) => write!(f, "{}", s),
        }
    }
}