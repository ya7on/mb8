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

        let unary = recursive(|unary| {
            just(TokenKind::OperatorAmpersand)
                .then(unary.clone())
                .map_with(|(_, expr), extra| {
                    let span: SimpleSpan = extra.span();
                    ASTExpr::AddressOf {
                        expr: Box::new(expr),
                        span: Span {
                            start: span.start,
                            end: span.end,
                        },
                    }
                })
                .or(just(TokenKind::OperatorAsterisk)
                    .then(unary.clone())
                    .map_with(|(_, expr), extra| {
                        let span: SimpleSpan = extra.span();
                        ASTExpr::Dereference {
                            expr: Box::new(expr),
                            span: Span {
                                start: span.start,
                                end: span.end,
                            },
                        }
                    }))
                .or(primary.clone())
        });

        let product = unary.clone().foldl_with(
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

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_expr(input: &[TokenKind]) -> ASTExpr {
        expr_parser().parse(input).unwrap()
    }

    #[test]
    fn test_plus() {
        let result = parse_expr(&[
            TokenKind::LiteralU8(1),
            TokenKind::OperatorPlus,
            TokenKind::LiteralU8(2),
        ]);
        assert_eq!(
            result,
            ASTExpr::BinaryOp {
                op: ASTBinaryOp::Add,
                lhs: Box::new(ASTExpr::LiteralU8 {
                    value: 1,
                    span: Span { start: 0, end: 1 }
                }),
                rhs: Box::new(ASTExpr::LiteralU8 {
                    value: 2,
                    span: Span { start: 2, end: 3 }
                }),
                span: Span { start: 0, end: 3 }
            }
        );
    }

    #[test]
    fn test_address_of() {
        let result = parse_expr(&[
            TokenKind::OperatorAmpersand,
            TokenKind::Ident("varname".to_string()),
        ]);
        assert_eq!(
            result,
            ASTExpr::AddressOf {
                expr: Box::new(ASTExpr::Var {
                    name: "varname".to_string(),
                    span: Span { start: 1, end: 2 }
                }),
                span: Span { start: 0, end: 2 }
            }
        );
    }

    #[test]
    fn test_dereference() {
        let result = parse_expr(&[
            TokenKind::OperatorAsterisk,
            TokenKind::Ident("varname".to_string()),
        ]);
        assert_eq!(
            result,
            ASTExpr::Dereference {
                expr: Box::new(ASTExpr::Var {
                    name: "varname".to_string(),
                    span: Span { start: 1, end: 2 }
                }),
                span: Span { start: 0, end: 2 }
            }
        );
    }
}
