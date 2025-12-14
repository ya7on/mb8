use chumsky::error::Simple;
use chumsky::extra::Err;
use chumsky::prelude::just;
use chumsky::IterParser;
use chumsky::{prelude::recursive, select, Parser};

use crate::ast::{ASTStmt, Span};
use crate::tokens::TokenKind;

use super::{expr::expr_parser, ty::ty_parser};

#[must_use]
pub fn stmt_parser<'src>(
) -> impl Parser<'src, &'src [TokenKind], ASTStmt, Err<Simple<'src, TokenKind>>> + Clone {
    recursive(|stmt| {
        let return_parser = just(TokenKind::KeywordReturn)
            .ignore_then(expr_parser().or_not())
            .then_ignore(just(TokenKind::Semicolon))
            .map_with(|expr, extra| {
                let span = extra.span();
                ASTStmt::Return {
                    expr,
                    span: Span {
                        start: span.start,
                        end: span.end,
                    },
                }
            });

        let declaration_parser = ty_parser()
            .then(select! {TokenKind::Ident(name) => name})
            .then(
                just(TokenKind::OperatorEq)
                    .ignore_then(expr_parser())
                    .or_not(),
            )
            .then_ignore(just(TokenKind::Semicolon))
            .map_with(|((ty, name), init), extra| {
                let span = extra.span();
                ASTStmt::Declaration {
                    name,
                    ty,
                    init,
                    span: Span {
                        start: span.start,
                        end: span.end,
                    },
                }
            });

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
            .map_with(|((condition, then_branch), else_branch), extra| {
                let span = extra.span();
                ASTStmt::If {
                    condition,
                    then_branch: Box::new(ASTStmt::Block(then_branch)),
                    else_branch: else_branch.map(ASTStmt::Block).map(Box::new),
                    span: Span {
                        start: span.start,
                        end: span.end,
                    },
                }
            });

        let while_parser = just(TokenKind::KeywordWhile)
            .ignore_then(expr_parser())
            .then(
                stmt.clone()
                    .repeated()
                    .collect()
                    .delimited_by(just(TokenKind::LeftBrace), just(TokenKind::RightBrace)),
            )
            .map_with(|(condition, body), extra| {
                let span = extra.span();
                ASTStmt::While {
                    condition,
                    body: Box::new(ASTStmt::Block(body)),
                    span: Span {
                        start: span.start,
                        end: span.end,
                    },
                }
            });

        let expr_parser = expr_parser()
            .then_ignore(just(TokenKind::Semicolon))
            .map_with(|expr, extra| {
                let span = extra.span();
                ASTStmt::Expression {
                    expr,
                    span: Span {
                        start: span.start,
                        end: span.end,
                    },
                }
            });

        return_parser
            .or(declaration_parser)
            .or(if_parser)
            .or(while_parser)
            .or(expr_parser)
    })
}
