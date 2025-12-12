use chumsky::{select, Parser};

use crate::{ast::Type, tokens::TokenKind};

#[must_use]
pub fn ty_parser<'src>() -> impl Parser<'src, &'src [TokenKind], Type> + Clone {
    select! {
        TokenKind::KeywordInt => Type::Int,
        TokenKind::KeywordChar => Type::Char,
        TokenKind::KeywordVoid => Type::Void,
    }
}
