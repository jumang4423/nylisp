pub const LPAREN: &str = "💖";
pub const RPAREN: &str = "💔";
pub const QUOTE: &str = "😪";
pub const TRUE: &str = "👍";
pub const FALSE: &str = "👎";
pub const IF: &str = "🐶";
pub const VAR: &str = "🌹";
pub const CLOSURE: &str = "🐷";
pub const SCOPED_LET: &str = "🍙";


pub struct Tokenizer {
    pub input: String,
}

impl Tokenizer {
    pub fn new(input: String) -> Tokenizer {
        Tokenizer { input }
    }

    pub fn tokenize(&self) -> Vec<String> {
        let paren_spaced = self.input
            .replace(LPAREN.chars().collect::<Vec<char>>()[0], format!(" {} ", LPAREN).as_str())
            .replace(RPAREN.chars().collect::<Vec<char>>()[0], format!(" {} ", RPAREN).as_str())
            .replace(QUOTE.chars().collect::<Vec<char>>()[0], format!(" {} ", QUOTE).as_str());
        paren_spaced.split_whitespace()
            .map(|x| x.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_nylisp_test1() {
        let input = "💖+ 1 2💔";
        let expected = vec!["💖", "+", "1", "2", "💔"];
        let tokenizer = Tokenizer::new(input.to_string());
        let tokens = tokenizer.tokenize();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn tokenize_nylisp_test2() {
        let input = "💖+ 💖* 2 3💔 💖* 4 5💔💔";
        let expected = vec!["💖", "+", "💖", "*", "2", "3", "💔", "💖", "*", "4", "5", "💔", "💔"];
        let tokenizer = Tokenizer::new(input.to_string());
        let tokens = tokenizer.tokenize();
        assert_eq!(tokens, expected);
    }
}