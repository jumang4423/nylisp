use wasm_bindgen_test::*;
// import Nylisp
use nylisp_wasm::*;

#[wasm_bindgen_test]
fn test_run() {
    let mut nylisp = NyLisp::new();
    let input = "💖🍙 💖💖x 2💔💖y 2💔💔 💖+ x y💔💔".to_string();
    let result = nylisp.run(input);
    assert_eq!(result[0].as_string().unwrap(), "4");
}