use crate::tokenizer::token::Token;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub position: usize,
}

impl Parser {
    #[must_use]
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }
}
