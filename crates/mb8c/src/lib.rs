use chumsky::Parser;
use parser::program::program_parser;

pub mod codegen;
pub mod error;
pub mod ir;
pub mod parser;
pub mod semantic;
pub mod tokenizer;

/// Compile the input string into an executable program.
///
/// # Errors
/// Returns an error if the input string is not valid MB8C code.
pub fn compile(input: &str) -> error::CompileResult<()> {
    let mut lexer = tokenizer::lexer::Lexer::new(input);
    let tokens = lexer.tokenize()?;
    let kinds = tokens.into_iter().map(|t| t.kind).collect::<Vec<_>>();
    let parser = program_parser();
    let ast = parser.parse(&kinds);

    println!("AST {ast:?}");

    // semantic::analyze(&ast)?;

    // let ir = lower_program(&ast)?;

    // let code = CodeGenerator::new(ir).generate()?;
    // println!("{code}");

    Ok(())
}
