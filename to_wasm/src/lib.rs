use wasm_bindgen::prelude::*;
pub use nylisp_eval;

#[wasm_bindgen]
pub struct NyLisp {
    global_env: nylisp_eval::ast::ast::Environment<'static>
}

// excute programs
#[wasm_bindgen]
impl NyLisp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> NyLisp {
        NyLisp {
            global_env: nylisp_eval::environment::environment::builtin_env()
        }
    }

    pub fn run(&mut self, _lines: String) -> Vec<JsValue> {
        let tokens = nylisp_eval::tokenize_nylisp(_lines);
        if tokens.len() == 0 {
            return vec![JsValue::from_str("ERR<tokenizer>: no input")];
        }
        let ast = nylisp_eval::parse_nylisps(tokens);
        let treed_tokens = match _parser_validator(ast) {
            Ok(tokens) => tokens,
            Err(err) => return vec![err]
        };
        let result = nylisp_eval::evaluate_nylisp(treed_tokens, &mut self.global_env);

        _evaluator_validator(result)
    }
}

fn _parser_validator(ast: Vec<Result<nylisp_eval::ast::ast::NylispExpression, nylisp_eval::ast::ast::NylispError>>) -> Result<Vec<nylisp_eval::ast::ast::NylispExpression>, JsValue> {
    let mut validated_ast: Vec<nylisp_eval::ast::ast::NylispExpression> = Vec::new();
    for expr in ast {
        match expr {
            Ok(expr) => validated_ast.push(expr),
            Err(err) => return Err(JsValue::from_str(&format!("ERR<parser>: {}", err)))
        }
    }
    Ok(validated_ast)
}

fn _evaluator_validator(ast: Vec<Result<nylisp_eval::ast::ast::NylispExpression, nylisp_eval::ast::ast::NylispError>>) -> Vec<JsValue> {
    let mut validated_ast: Vec<JsValue> = Vec::new();
    for expr in ast {
        match expr {
            Ok(expr) => validated_ast.push(JsValue::from_str(&format!("{}", expr))),
            Err(err) => return vec![JsValue::from_str(&format!("ERR<evaluator>: {}", err))]
        }
    }

    validated_ast
}