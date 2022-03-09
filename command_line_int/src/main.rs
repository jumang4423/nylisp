pub use nylisp_eval;

fn main() {
    let tokens = nylisp_eval::tokenize_nylisp("💖+ 1 2💔 💖+ 1 2💔".to_string());
    println!("{:?}", tokens);
    let ast = nylisp_eval::parse_nylisps(tokens);
    println!("{:?}", ast);
    let mut validated_ast: Vec<nylisp_eval::ast::ast::NylispExpression> = Vec::new();
    for expr in ast {
        match expr {
            nylisp_eval::ast::ast::NylispExpression(expr) => {
                validated_ast.push(expr);
            }
            _ => {}
        }
    }
    let result = nylisp_eval::evaluate_nylisp(validated_ast);
}

