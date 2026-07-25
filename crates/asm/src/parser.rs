use chumsky::error::Rich;
use chumsky::prelude::{choice, just};
use chumsky::{
    extra::Err,
    input::{Input, ValueInput},
    span::SimpleSpan,
    Parser,
};
use chumsky::{select, IterParser};
use mb8_isa::registers::Register;

use crate::ast::{ASTInstruction, ASTItem, ASTProgram, DataSource, Directive, Operand};
use crate::diagnostics::{Diagnostic, DiagnosticKind, Severity, SourceId, Span, Spanned};
use crate::pass::{AssemblerPass, PassContext};
use crate::tokens::{TokenKind, TokenStream};

#[must_use]
#[allow(clippy::too_many_lines)]
fn build_parser<'src, I>(
    source: SourceId,
) -> impl Parser<'src, I, ASTProgram, Err<Rich<'src, TokenKind>>>
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan<usize>>,
{
    let label_define = select! {
        TokenKind::Ident(label) => label,
    }
    .then_ignore(just(TokenKind::Colon))
    .map_with(move |label, extra| {
        let span: SimpleSpan<usize> = extra.span();
        ASTItem::Label(Spanned {
            value: label,
            span: Span {
                source,
                range: span.start..span.end,
            },
        })
    });

    let register_parser = select! {
        TokenKind::Ident(name) if name == "r0" => Register::R0,
        TokenKind::Ident(name) if name == "r1" => Register::R1,
        TokenKind::Ident(name) if name == "r2" => Register::R2,
        TokenKind::Ident(name) if name == "r3" => Register::R3,
        TokenKind::Ident(name) if name == "r4" => Register::R4,
        TokenKind::Ident(name) if name == "r5" => Register::R5,
        TokenKind::Ident(name) if name == "r6" => Register::R6,
        TokenKind::Ident(name) if name == "r7" => Register::R7,
        TokenKind::Ident(name) if name == "r8" => Register::R8,
        TokenKind::Ident(name) if name == "r9" => Register::R9,
        TokenKind::Ident(name) if name == "r10" => Register::R10,
        TokenKind::Ident(name) if name == "r11" => Register::R11,
        TokenKind::Ident(name) if name == "r12" => Register::R12,
        TokenKind::Ident(name) if name == "r13" => Register::R13,
        TokenKind::Ident(name) if name == "r14" => Register::R14,
        TokenKind::Ident(name) if name == "r15" => Register::R15,

        TokenKind::Ident(name) if name == "a" => Register::A,
        TokenKind::Ident(name) if name == "ih" => Register::IH,
        TokenKind::Ident(name) if name == "il" => Register::IL,
        TokenKind::Ident(name) if name == "fph" => Register::FPH,
        TokenKind::Ident(name) if name == "fpl" => Register::FPL,
        TokenKind::Ident(name) if name == "sph" => Register::SPH,
        TokenKind::Ident(name) if name == "spl" => Register::SPL,
        TokenKind::Ident(name) if name == "f" => Register::F,
    };
    let register = register_parser.map(DataSource::Register);
    let register_pair = register_parser
        .then_ignore(just(TokenKind::Colon))
        .then(register_parser)
        .map(|(register1, register2)| DataSource::RegisterPair(register1, register2));
    let register_pair_offset = register_parser
        .then_ignore(just(TokenKind::Colon))
        .then(register_parser)
        .then_ignore(just(TokenKind::Minus))
        .then(select! {
            TokenKind::Integer(value) => value,
        })
        .try_map_with(|((hi, lo), offset), extra| {
            let span: SimpleSpan<usize> = extra.span();
            let offset = u8::try_from(offset)
                .map_err(|_| Rich::custom(span, "Offset does not fit in u8"))?;
            Ok(Operand::MemoryOffset { hi, lo, offset })
        });
    let immediate = select! {
        TokenKind::Integer(value) => value,
    }
    .map(DataSource::Immediate);
    let label_reference = select! {
        TokenKind::Ident(label) => label,
    }
    .map_with(move |label, extra| {
        let span: SimpleSpan<usize> = extra.span();
        DataSource::Label(Spanned {
            value: label,
            span: Span {
                source,
                range: span.start..span.end,
            },
        })
    });

    let operand = choice((
        register_pair_offset
            .delimited_by(just(TokenKind::LeftBracket), just(TokenKind::RightBracket)),
        register_pair.clone().map(Operand::Raw),
        register.map(Operand::Raw),
        immediate.map(Operand::Raw),
        label_reference.map(Operand::Raw),
        register
            .delimited_by(just(TokenKind::LeftBracket), just(TokenKind::RightBracket))
            .map(Operand::MemoryWrapped),
        register_pair
            .delimited_by(just(TokenKind::LeftBracket), just(TokenKind::RightBracket))
            .map(Operand::MemoryWrapped),
        immediate
            .delimited_by(just(TokenKind::LeftBracket), just(TokenKind::RightBracket))
            .map(Operand::MemoryWrapped),
        label_reference
            .delimited_by(just(TokenKind::LeftBracket), just(TokenKind::RightBracket))
            .map(Operand::MemoryWrapped),
    ));

    let instruction = select! {
        TokenKind::Ident(name) => name,
    }
    .then(
        operand
            .separated_by(just(TokenKind::Comma))
            .collect::<Vec<_>>(),
    )
    .map_with(move |(mnemonic, operands), extra| {
        let span: SimpleSpan<usize> = extra.span();
        ASTItem::Instruction(Spanned {
            value: ASTInstruction { mnemonic, operands },
            span: Span {
                source,
                range: span.start..span.end,
            },
        })
    });

    let origin_directive = just(TokenKind::Dot)
        .ignore_then(just(TokenKind::Ident("origin".to_string())))
        .ignore_then(select! {
            TokenKind::Integer(number) => number,
        })
        .map(Directive::Origin);
    let address_directive = just(TokenKind::Dot)
        .ignore_then(just(TokenKind::Ident("addr".to_string())))
        .ignore_then(select! {
            TokenKind::Integer(number) => number,
        })
        .map(Directive::Address);
    let data_directive = just(TokenKind::Dot)
        .ignore_then(just(TokenKind::Ident("data".to_string())))
        .ignore_then(
            select! {
                TokenKind::Integer(number) => number,
            }
            .try_map_with(|byte, extra| {
                let span: SimpleSpan<usize> = extra.span();
                u8::try_from(byte).map_err(|_| Rich::custom(span, "Value out of range"))
            })
            .separated_by(just(TokenKind::Comma))
            .collect::<Vec<_>>(),
        )
        .map(Directive::Data);
    let include_directive = just(TokenKind::Dot)
        .ignore_then(just(TokenKind::Ident("include".to_string())))
        .ignore_then(
            select! {
                TokenKind::StringLiteral(path) => path,
            }
            .map(Directive::Include),
        );
    let ascii_directive = just(TokenKind::Dot)
        .ignore_then(just(TokenKind::Ident("ascii".to_string())))
        .ignore_then(select! {
            TokenKind::StringLiteral(value) => value,
        })
        .map(Directive::Ascii);

    let directive = choice((
        origin_directive,
        address_directive,
        data_directive,
        include_directive,
        ascii_directive,
    ))
    .map_with(move |directive, extra| {
        let span: SimpleSpan<usize> = extra.span();
        ASTItem::Directive(Spanned {
            value: directive,
            span: Span {
                source,
                range: span.start..span.end,
            },
        })
    });

    let item = choice((label_define, instruction, directive));
    item.separated_by(just(TokenKind::Newline).repeated().at_least(1))
        .allow_leading()
        .allow_trailing()
        .collect::<Vec<_>>()
        .map(|items| ASTProgram { items })
}

pub(crate) struct ParsePass;

impl AssemblerPass for ParsePass {
    type Input = TokenStream;
    type Output = ASTProgram;

    fn run(&mut self, input: Self::Input, context: &mut PassContext<'_>) -> Option<Self::Output> {
        let eoi = SimpleSpan::from(input.end..input.end);
        let tokens = input
            .tokens
            .into_iter()
            .filter(|(token, _)| !matches!(token, TokenKind::Comment(_)))
            .collect::<Vec<_>>();
        let token_input = tokens.as_slice().split_token_span(eoi);
        let parsed = build_parser(input.source).parse(token_input).into_result();
        match parsed {
            Ok(ast) => Some(ast),
            Err(errors) => {
                let diagnostics = errors
                    .into_iter()
                    .map(|error| {
                        let span = *error.span();
                        Diagnostic {
                            severity: Severity::Error,
                            span: Some(Span {
                                source: input.source,
                                range: span.start..span.end,
                            }),
                            kind: DiagnosticKind::Parse {
                                message: format!("{error}"),
                            },
                        }
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
