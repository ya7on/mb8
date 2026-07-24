use chumsky::error::Rich;
use chumsky::prelude::{choice, end, just, none_of, one_of};
use chumsky::{extra::Err, span::SimpleSpan, Parser};
use chumsky::{text, IterParser};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Ident(String),
    StringLiteral(String),
    Integer(u16),
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Dot,
    Minus,
    Newline,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ident(value) => write!(f, "{value}"),
            Self::StringLiteral(value) => write!(f, "\"{value}\""),
            Self::Integer(value) => write!(f, "{value:#x}"),
            Self::LeftBracket => write!(f, "["),
            Self::RightBracket => write!(f, "]"),
            Self::Comma => write!(f, ","),
            Self::Colon => write!(f, ":"),
            Self::Dot => write!(f, "."),
            Self::Minus => write!(f, "-"),
            Self::Newline => writeln!(f),
        }
    }
}

#[must_use]
pub fn lexer<'src>(
) -> impl Parser<'src, &'src str, Vec<(TokenKind, SimpleSpan<usize>)>, Err<Rich<'src, char>>> {
    let ident = text::ascii::ident().map(|ident: &'src str| TokenKind::Ident(ident.to_lowercase()));

    let string = none_of('"')
        .repeated()
        .to_slice()
        .delimited_by(just('"'), just('"'))
        .map(|value: &'src str| TokenKind::StringLiteral(value.to_owned()));

    let hex_number =
        just("0x")
            .ignore_then(text::digits(16).to_slice())
            .try_map(|digits: &str, span| {
                u16::from_str_radix(digits, 16)
                    .map(TokenKind::Integer)
                    .map_err(|_| Rich::custom(span, "HEX number does not fit in u16"))
            });

    let punctuation = choice((
        just('[').to(TokenKind::LeftBracket),
        just(']').to(TokenKind::RightBracket),
        just(',').to(TokenKind::Comma),
        just(':').to(TokenKind::Colon),
        just('.').to(TokenKind::Dot),
        just('-').to(TokenKind::Minus),
        just('\n').to(TokenKind::Newline),
    ));

    let token = choice((ident, string, hex_number, punctuation))
        .map_with(|token, e| (token, e.span()))
        .padded_by(one_of(" \t\r").repeated());

    token.repeated().collect().then_ignore(end())
}
