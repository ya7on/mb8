use codegen::CodeGenerator;
use ir::lower_program;

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
    let mut parser = parser::base::Parser::new(tokens);
    let ast = parser.parse_program()?;

    semantic::analyze(&ast)?;

    let ir = lower_program(&ast)?;

    let code = CodeGenerator::new(ir).generate()?;
    println!("{code}");

    Ok(())
}
