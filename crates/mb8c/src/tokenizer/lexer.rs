use crate::{
    error::{CompileError, CompileResult},
    tokenizer::helpers::is_ident,
};

use super::{
    helpers::is_digit,
    token::{Keyword, Span, Token, TokenKind},
};

#[derive(Debug)]
/// Tokenizer for the subset of C language
pub struct Lexer<'a> {
    /// File input in bytes
    pub input: &'a [u8],
    /// Length of the input
    pub length: usize,
    /// Current position in the input
    pub position: usize,
    /// Current line number
    pub line: usize,
    /// Current column number
    pub column: usize,
}

impl<'a> Lexer<'a> {
    #[must_use]
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.as_bytes(),
            length: input.len(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenize the input
    ///
    /// # Errors
    ///
    /// Returns an error if the input is invalid
    pub fn tokenize(&mut self) -> CompileResult<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;
            let eof = matches!(token.kind, TokenKind::Eof);
            tokens.push(token);
            if eof {
                break;
            }
        }

        Ok(tokens)
    }

    /// Advance the lexer to the next token
    ///
    /// # Errors
    ///
    /// Returns an error if the input is invalid
    pub fn next_token(&mut self) -> CompileResult<Token> {
        self.skip_whitespace();

        let start = self.position;
        let line = self.line;
        let column = self.column;

        let kind = 'kind: {
            let Some(char) = self.peek() else {
                break 'kind TokenKind::Eof;
            };

            if is_ident(char) {
                let mut ident = String::new();

                while let Some(c) = self.peek() {
                    if is_ident(c) || is_digit(c) {
                        ident.push(c);
                        self.bump();
                    } else {
                        break;
                    }
                }

                break 'kind match ident.as_str() {
                    "void" => TokenKind::Keyword(Keyword::Void),
                    "char" => TokenKind::Keyword(Keyword::Char),
                    "int" => TokenKind::Keyword(Keyword::Int),
                    "if" => TokenKind::Keyword(Keyword::If),
                    "else" => TokenKind::Keyword(Keyword::Else),
                    "while" => TokenKind::Keyword(Keyword::While),
                    "return" => TokenKind::Keyword(Keyword::Return),
                    _ => TokenKind::Ident(ident),
                };
            }

            if is_digit(char) {
                let mut number = String::new();

                while let Some(c) = self.peek() {
                    if is_digit(c) {
                        number.push(c);
                        self.bump();
                    } else {
                        break;
                    }
                }

                break 'kind TokenKind::Number(
                    number
                        .parse()
                        .map_err(|_| CompileError::UnexpectedToken { line, column })?,
                );
            }

            // Operators
            if let Some(operator) = self.parse_operator(char) {
                break 'kind TokenKind::Operator(operator);
            }

            // Delimiters
            match char {
                '{' => {
                    self.bump();
                    break 'kind TokenKind::LeftBrace;
                }
                '}' => {
                    self.bump();
                    break 'kind TokenKind::RightBrace;
                }
                '(' => {
                    self.bump();
                    break 'kind TokenKind::LeftParenthesis;
                }
                ')' => {
                    self.bump();
                    break 'kind TokenKind::RightParenthesis;
                }
                ',' => {
                    self.bump();
                    break 'kind TokenKind::Comma;
                }
                ';' => {
                    self.bump();
                    break 'kind TokenKind::Semicolon;
                }
                _ => {}
            }

            return Err(CompileError::UnexpectedToken { column, line });
        };

        let end = self.position;

        Ok(Token {
            kind,
            span: Span {
                start,
                end,
                line,
                column,
            },
        })
    }
}
