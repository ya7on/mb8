use chumsky::error::Simple;
use chumsky::extra::Err;
use chumsky::input::ValueInput;
use chumsky::prelude::just;
use chumsky::span::SimpleSpan;
use chumsky::{prelude::recursive, Parser};
use chumsky::{select, IterParser};

use crate::lex::tokens::TokenKind;
use crate::parser::ast::{ASTStmt, Span};

use super::parse_expr::expr_parser;

#[must_use]
pub fn stmt_parser<'src, I>() -> impl Parser<'src, I, ASTStmt, Err<Simple<'src, TokenKind>>> + Clone
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    recursive(|stmt| {
        let return_parser = just(TokenKind::KeywordReturn)
            .ignore_then(expr_parser().or_not())
            .then_ignore(just(TokenKind::Semicolon))
            .map_with(|expr, extra| {
                let span: SimpleSpan = extra.span();
                ASTStmt::Return {
                    expr,
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
            .delimited_by(just(TokenKind::KeywordBegin), just(TokenKind::KeywordEnd));

        let if_parser = just(TokenKind::KeywordIf)
            .ignore_then(expr_parser().delimited_by(
                just(TokenKind::LeftParenthesis),
                just(TokenKind::RightParenthesis),
            ))
            .then_ignore(just(TokenKind::KeywordThen))
            .then(block_parser.clone())
            .then(
                just(TokenKind::KeywordElse)
                    .ignore_then(block_parser.clone())
                    .or_not(),
            )
            .map_with(|((condition, then_branch), else_branch), extra| {
                let span: SimpleSpan = extra.span();
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
            .then_ignore(just(TokenKind::KeywordDo))
            .then(
                stmt.clone()
                    .repeated()
                    .collect()
                    .delimited_by(just(TokenKind::KeywordBegin), just(TokenKind::KeywordEnd)),
            )
            .map_with(|(condition, body), extra| {
                let span: SimpleSpan = extra.span();
                ASTStmt::While {
                    condition,
                    body: Box::new(ASTStmt::Block(body)),
                    span: Span {
                        start: span.start,
                        end: span.end,
                    },
                }
            });

        let assign_parser = select! { TokenKind::Ident(name) => name }
            .then_ignore(just(TokenKind::OperatorEq))
            .then(expr_parser())
            .then_ignore(just(TokenKind::Semicolon))
            .map_with(|(name, value), extra| {
                let span: SimpleSpan = extra.span();
                ASTStmt::Assign {
                    name,
                    value,
                    span: Span {
                        start: span.start,
                        end: span.end,
                    },
                }
            });

        let expr_parser = expr_parser()
            .then_ignore(just(TokenKind::Semicolon))
            .map_with(|expr, extra| {
                let span: SimpleSpan = extra.span();
                ASTStmt::Expression {
                    expr,
                    span: Span {
                        start: span.start,
                        end: span.end,
                    },
                }
            });

        return_parser
            .or(if_parser)
            .or(while_parser)
            .or(assign_parser)
            .or(expr_parser)
    })
}
