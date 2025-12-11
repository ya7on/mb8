use chumsky::prelude::just;
use chumsky::IterParser;
use chumsky::{prelude::recursive, select, Parser};

use crate::{
    parser::ast::Stmt,
    tokenizer::token::{Keyword, Operator, TokenKind},
};

use super::{expr::expr_parser, ty::ty_parser};

pub fn stmt_parser<'src>() -> impl Parser<'src, &'src [TokenKind], Stmt> + Clone {
    recursive(|stmt| {
        let return_parser = just(TokenKind::Keyword(Keyword::Return))
            .ignore_then(expr_parser().or_not())
            .then_ignore(just(TokenKind::Semicolon))
            .map(|expr| Stmt::Return(expr));

        let declaration_parser = ty_parser()
            .then(select! {TokenKind::Ident(name) => name})
            .then(
                just(TokenKind::Operator(Operator::Eq))
                    .ignore_then(expr_parser())
                    .or_not(),
            )
            .then_ignore(just(TokenKind::Semicolon))
            .map(|((ty, name), init)| Stmt::Declaration { name, ty, init });

        let if_parser = just(TokenKind::Keyword(Keyword::If))
            .ignore_then(expr_parser().delimited_by(
                just(TokenKind::LeftParenthesis),
                just(TokenKind::RightParenthesis),
            ))
            .then(
                stmt.clone()
                    .delimited_by(just(TokenKind::LeftBrace), just(TokenKind::RightBrace)),
            )
            .then(
                just(TokenKind::Keyword(Keyword::Else))
                    .ignore_then(
                        stmt.clone()
                            .delimited_by(just(TokenKind::LeftBrace), just(TokenKind::RightBrace)),
                    )
                    .or_not(),
            )
            .map(|((condition, then_branch), else_branch)| Stmt::If {
                condition,
                then_branch: Box::new(then_branch),
                else_branch: else_branch.map(Box::new),
            });

        let while_parser = just(TokenKind::Keyword(Keyword::While))
            .ignore_then(expr_parser())
            .then(
                stmt.clone()
                    .repeated()
                    .collect()
                    .delimited_by(just(TokenKind::LeftBrace), just(TokenKind::RightBrace)),
            )
            .map(|(condition, body)| Stmt::While {
                condition,
                body: Box::new(Stmt::Block(body)),
            });

        return_parser
            .or(declaration_parser)
            .or(if_parser)
            .or(while_parser)
    })
}
