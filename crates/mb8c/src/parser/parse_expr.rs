use chumsky::{
    error::Simple,
    extra::Err,
    input::ValueInput,
    prelude::{just, recursive},
    select,
    span::SimpleSpan,
    IterParser, Parser,
};

use crate::{
    lex::tokens::TokenKind,
    parser::ast::{ASTBinaryOp, ASTExpr, Span},
};

#[must_use]
#[allow(clippy::too_many_lines)]
pub fn expr_parser<'src, I>() -> impl Parser<'src, I, ASTExpr, Err<Simple<'src, TokenKind>>> + Clone
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    recursive(|expr| {
        let args = expr
            .clone()
            .separated_by(just(TokenKind::Comma))
            .collect::<Vec<_>>()
            .delimited_by(
                just(TokenKind::LeftParenthesis),
                just(TokenKind::RightParenthesis),
            );

        let call_expr = select! {
            TokenKind::Ident(name) => name,
        }
        .then(args.clone())
        .map_with(|(name, args), extra| {
            let span: SimpleSpan = extra.span();
            ASTExpr::Call {
                name,
                args,
                span: Span {
                    start: span.start,
                    end: span.end,
                },
            }
        });

        let primary = call_expr
            .or(
                select! { TokenKind::LiteralU8(number) => number }.map_with(|number, extra| {
                    let span: SimpleSpan = extra.span();
                    ASTExpr::LiteralU8 {
                        value: number,
                        span: Span {
                            start: span.start,
                            end: span.end,
                        },
                    }
                }),
            )
            .or(
                select! { TokenKind::LiteralU16(number) => number }.map_with(|number, extra| {
                    let span: SimpleSpan = extra.span();
                    ASTExpr::LiteralU16 {
                        value: number,
                        span: Span {
                            start: span.start,
                            end: span.end,
                        },
                    }
                }),
            )
            .or(
                select! { TokenKind::Ident(name) => name }.map_with(|name, extra| {
                    let span: SimpleSpan = extra.span();
                    ASTExpr::Var {
                        name,
                        span: Span {
                            start: span.start,
                            end: span.end,
                        },
                    }
                }),
            )
            .or(expr.clone().delimited_by(
                just(TokenKind::LeftParenthesis),
                just(TokenKind::RightParenthesis),
            ));

        let product = primary.clone().foldl_with(
            (just(TokenKind::OperatorAsterisk)
                .to(ASTBinaryOp::Mul)
                .or(just(TokenKind::OperatorSlash).to(ASTBinaryOp::Div)))
            .then(primary.clone())
            .repeated(),
            |lhs, (op, rhs), extra| {
                let span: SimpleSpan = extra.span();
                ASTExpr::BinaryOp {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    span: Span {
                        start: span.start,
                        end: span.end,
                    },
                }
            },
        );

        let sum = product.clone().foldl_with(
            (just(TokenKind::OperatorPlus)
                .to(ASTBinaryOp::Add)
                .or(just(TokenKind::OperatorMinus).to(ASTBinaryOp::Sub)))
            .then(product)
            .repeated(),
            |lhs, (op, rhs), extra| {
                let span: SimpleSpan = extra.span();
                ASTExpr::BinaryOp {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    span: Span {
                        start: span.start,
                        end: span.end,
                    },
                }
            },
        );

        let equality = sum.clone().foldl_with(
            just(TokenKind::OperatorEqEq)
                .to(ASTBinaryOp::Eq)
                .then(sum)
                .repeated(),
            |lhs, (op, rhs), extra| {
                let span: SimpleSpan = extra.span();
                ASTExpr::BinaryOp {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    span: Span {
                        start: span.start,
                        end: span.end,
                    },
                }
            },
        );

        equality
    })
}
