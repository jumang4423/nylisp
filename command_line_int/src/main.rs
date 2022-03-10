pub use nylisp_eval;

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn main() {
    let mut global_env = nylisp_eval::environment::environment::builtin_env();
    println!("-o welcome to nylisp repl");
    println!("-! ctrl+d to exit");
    loop {
        println!("*");
        let tokens = nylisp_eval::tokenize_nylisp(read_line());
        if tokens.len() == 0 {
            continue;
        }
        let ast = nylisp_eval::parse_nylisps(tokens);
        let mut validated_ast: Vec<nylisp_eval::ast::ast::NylispExpression> = Vec::new();
        for expr in ast {
            match expr {
                Ok(expr) => validated_ast.push(expr),
                Err(err) => println!("{:?}", err),
            }
        }
        let result = nylisp_eval::evaluate_nylisp(validated_ast, &mut global_env);

        println!("{:?}", result);
    }
}

