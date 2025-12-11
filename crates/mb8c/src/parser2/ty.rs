use chumsky::{select, Parser};

use crate::{
    parser::ast::Type,
    tokenizer::token::{Keyword, TokenKind},
};

pub fn ty_parser<'src>() -> impl Parser<'src, &'src [TokenKind], Type> {
    select! {
        TokenKind::Keyword(Keyword::Int) => Type::Int,
        TokenKind::Keyword(Keyword::Char) => Type::Char,
        TokenKind::Keyword(Keyword::Void) => Type::Void,
    }
}
