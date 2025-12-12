use chumsky::prelude::just;
use chumsky::IterParser;
use chumsky::{prelude::recursive, select, Parser};

use crate::ast::Stmt;
use crate::tokens::TokenKind;

use super::{expr::expr_parser, ty::ty_parser};

#[must_use]
pub fn stmt_parser<'src>() -> impl Parser<'src, &'src [TokenKind], Stmt> + Clone {
    recursive(|stmt| {
        let return_parser = just(TokenKind::KeywordReturn)
            .ignore_then(expr_parser().or_not())
            .then_ignore(just(TokenKind::Semicolon))
            .map(Stmt::Return);

        let declaration_parser = ty_parser()
            .then(select! {TokenKind::Ident(name) => name})
            .then(
                just(TokenKind::OperatorEq)
                    .ignore_then(expr_parser())
                    .or_not(),
            )
            .then_ignore(just(TokenKind::Semicolon))
            .map(|((ty, name), init)| Stmt::Declaration { name, ty, init });

        let block_parser = stmt
            .clone()
            .repeated()
            .collect::<Vec<_>>()
            .delimited_by(just(TokenKind::LeftBrace), just(TokenKind::RightBrace));

        let if_parser = just(TokenKind::KeywordIf)
            .ignore_then(expr_parser().delimited_by(
                just(TokenKind::LeftParenthesis),
                just(TokenKind::RightParenthesis),
            ))
            .then(block_parser.clone())
            .then(
                just(TokenKind::KeywordElse)
                    .ignore_then(block_parser.clone())
                    .or_not(),
            )
            .map(|((condition, then_branch), else_branch)| Stmt::If {
                condition,
                then_branch: Box::new(Stmt::Block(then_branch)),
                else_branch: else_branch.map(Stmt::Block).map(Box::new),
            });

        let while_parser = just(TokenKind::KeywordWhile)
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

        let expr_parser = expr_parser()
            .then_ignore(just(TokenKind::Semicolon))
            .map(Stmt::Expression);

        return_parser
            .or(declaration_parser)
            .or(if_parser)
            .or(while_parser)
            .or(expr_parser)
    })
}
