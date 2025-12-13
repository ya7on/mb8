use chumsky::{select, Parser};

use crate::{ast::ASTType, tokens::TokenKind};

#[must_use]
pub fn ty_parser<'src>() -> impl Parser<'src, &'src [TokenKind], ASTType> + Clone {
    select! {
        TokenKind::KeywordInt => ASTType::Int,
        TokenKind::KeywordChar => ASTType::Char,
        TokenKind::KeywordVoid => ASTType::Void,
    }
}
