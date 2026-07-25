use chumsky::error::Rich;
use chumsky::prelude::{choice, end, just, none_of, one_of};
use chumsky::{extra::Err, span::SimpleSpan, Parser};
use chumsky::{text, IterParser};
use std::fmt;

use crate::{
    diagnostics::{Diagnostic, DiagnosticKind, Severity, SourceId, Span},
    pass::{AssemblerPass, PassContext},
};

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
    Comment(String),
    Newline,
}

#[derive(Debug)]
pub(crate) struct TokenStream {
    pub source: SourceId,
    pub end: usize,
    pub tokens: Vec<(TokenKind, SimpleSpan<usize>)>,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ident(value) => write!(f, "{value}"),
            Self::StringLiteral(value) => write!(f, "\"{}\"", value.escape_debug()),
            Self::Integer(value) => write!(f, "{value:#x}"),
            Self::LeftBracket => write!(f, "["),
            Self::RightBracket => write!(f, "]"),
            Self::Comma => write!(f, ","),
            Self::Colon => write!(f, ":"),
            Self::Dot => write!(f, "."),
            Self::Minus => write!(f, "-"),
            Self::Comment(value) => write!(f, ";{value}"),
            Self::Newline => writeln!(f),
        }
    }
}

#[must_use]
fn build_lexer<'src>(
) -> impl Parser<'src, &'src str, Vec<(TokenKind, SimpleSpan<usize>)>, Err<Rich<'src, char>>> {
    let ident = text::ascii::ident().map(|ident: &'src str| TokenKind::Ident(ident.to_lowercase()));

    let escape = just('\\').ignore_then(choice((
        just('n').to('\n'),
        just('0').to('\0'),
        just('\\').to('\\'),
        just('"').to('"'),
    )));
    let string = choice((escape, none_of("\\\"")))
        .repeated()
        .collect::<String>()
        .delimited_by(just('"'), just('"'))
        .map(TokenKind::StringLiteral);

    let hex_number =
        just("0x")
            .ignore_then(text::digits(16).to_slice())
            .try_map(|digits: &str, span| {
                u16::from_str_radix(digits, 16)
                    .map(TokenKind::Integer)
                    .map_err(|_| Rich::custom(span, "HEX number does not fit in u16"))
            });

    let comment = just(';')
        .ignore_then(none_of("\r\n").repeated().to_slice())
        .map(|value: &'src str| TokenKind::Comment(value.to_owned()));

    let punctuation = choice((
        just('[').to(TokenKind::LeftBracket),
        just(']').to(TokenKind::RightBracket),
        just(',').to(TokenKind::Comma),
        just(':').to(TokenKind::Colon),
        just('.').to(TokenKind::Dot),
        just('-').to(TokenKind::Minus),
        just('\n').to(TokenKind::Newline),
    ));

    let token = choice((ident, string, hex_number, comment, punctuation))
        .map_with(|token, e| (token, e.span()))
        .padded_by(one_of(" \t\r").repeated());

    token.repeated().collect().then_ignore(end())
}

pub(crate) struct LexPass;

impl AssemblerPass for LexPass {
    type Input = SourceId;
    type Output = TokenStream;

    fn run(
        &mut self,
        source_id: Self::Input,
        context: &mut PassContext<'_>,
    ) -> Option<Self::Output> {
        let (end, parsed) = {
            let source = context.source(source_id)?;
            (
                source.source.len(),
                build_lexer().parse(source.source.as_str()),
            )
        };

        match parsed.into_result() {
            Ok(tokens) => Some(TokenStream {
                source: source_id,
                end,
                tokens,
            }),
            Err(errors) => {
                let diagnostics = errors
                    .into_iter()
                    .map(|error| Diagnostic {
                        severity: Severity::Error,
                        span: Some(Span {
                            source: source_id,
                            range: error.span().into_range(),
                        }),
                        kind: DiagnosticKind::Lex {
                            message: format!("{error}"),
                        },
                    })
                    .collect::<Vec<_>>();
                for diagnostic in diagnostics {
                    context.emit_fatal(diagnostic);
                }
                None
            }
        }
    }
}
