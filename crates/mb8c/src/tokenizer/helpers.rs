use super::{lexer::Lexer, token::Operator};

impl Lexer<'_> {
    #[must_use]
    pub fn peek(&self) -> Option<char> {
        if self.position == self.input.len() {
            return None;
        }

        Some(self.input[self.position] as char)
    }

    #[must_use]
    pub fn peek_next(&self) -> Option<char> {
        if self.position + 1 >= self.input.len() {
            return None;
        }

        Some(self.input[self.position + 1] as char)
    }

    pub fn bump(&mut self) -> Option<char> {
        let char = self.peek()?;
        self.position += 1;
        if char == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(char)
    }

    pub fn bump_line(&mut self) {
        while let Some(char) = self.bump() {
            if char == '\n' {
                break;
            }
        }
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(char) = self.peek() {
            if char.is_whitespace() {
                self.bump();
                continue;
            }

            if char == '/' && self.peek_next() == Some('/') {
                self.bump_line();
                continue;
            }

            break;
        }
    }

    pub fn parse_operator(&mut self, c: char) -> Option<Operator> {
        if let ('=', Some('=')) = (c, self.peek_next()) {
            self.bump();
            self.bump();
            return Some(Operator::EqEq);
        }

        match c {
            '+' => {
                self.bump();
                Some(Operator::Plus)
            }
            '-' => {
                self.bump();
                Some(Operator::Minus)
            }
            '*' => {
                self.bump();
                Some(Operator::Asterisk)
            }
            '/' => {
                self.bump();
                Some(Operator::Slash)
            }
            '=' => {
                self.bump();
                Some(Operator::Eq)
            }
            _ => None,
        }
    }
}

#[must_use]
pub fn is_ident(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

#[must_use]
pub fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}
