use symbols::{FunctionSignature, Functions, Symbols};

use crate::{
    ast::{ASTExpr, ASTFunction, ASTProgram, ASTStmt, ASTType},
    error::{CompileError, CompileResult},
};

pub mod symbols;

/// Analyzes the given program.
///
/// # Errors
/// Returns an error if the program contains duplicate functions or if a function is not defined.
pub fn analyze(ast: &ASTProgram) -> CompileResult<()> {
    let mut functions = Functions::new();

    for function in &ast.functions {
        if functions.contains_key(&function.name) {
            return Err(todo!());
        }
        functions.insert(
            function.name.clone(),
            FunctionSignature {
                param_types: function.params.iter().map(|param| param.1).collect(),
                return_type: function.return_type,
            },
        );
    }

    for function in &ast.functions {
        analyze_function(&functions, function)?;
    }

    Ok(())
}

/// Analyzes a single function.
///
/// # Errors
/// Returns an error if the function contains duplicate symbols or if a symbol is not defined.
pub fn analyze_function(functions: &Functions, input: &ASTFunction) -> CompileResult<()> {
    let mut symbols = Symbols::default();

    for param in &input.params {
        symbols.insert(param.0.clone(), param.1)?;
    }

    analyze_stmt(functions, &mut symbols, input.return_type, &input.body)?;

    Ok(())
}

/// Analyzes a single statement.
///
/// # Errors
/// Returns an error if the statement contains duplicate symbols or if a symbol is not defined.
#[allow(clippy::too_many_lines)]
pub fn analyze_stmt(
    functions: &Functions,
    symbols: &mut Symbols,
    return_type: ASTType,
    stmt: &ASTStmt,
) -> CompileResult<()> {
    match stmt {
        ASTStmt::Block(stmts) => {
            symbols.enter_scope();
            for stmt in stmts {
                analyze_stmt(functions, symbols, return_type, stmt)?;
            }
            symbols.leave_scope();
            Ok(())
        }
        ASTStmt::Return(Some(expr)) => {
            let ty = analyze_expr(functions, symbols, expr)?;
            if ty != return_type {
                return Err(todo!());
            }
            Ok(())
        }
        ASTStmt::Return(None) => {
            if return_type == ASTType::Void {
                Ok(())
            } else {
                Err(todo!())
            }
        }
        ASTStmt::Expression(expr) => {
            analyze_expr(functions, symbols, expr)?;
            Ok(())
        }
        ASTStmt::Declaration { name, ty, init: _ } => {
            symbols.insert(name.to_owned(), *ty)?;
            Ok(())
        }
        ASTStmt::If {
            condition,
            then_branch,
            else_branch,
        } => {
            match condition {
                ASTExpr::IntLiteral(_) => {}
                ASTExpr::BinaryOp { op: _, lhs, rhs } => {
                    let lhs = analyze_expr(functions, symbols, lhs)?;
                    let rhs = analyze_expr(functions, symbols, rhs)?;
                    if lhs != rhs {
                        return Err(todo!());
                    }
                }
                ASTExpr::Var(name) => {
                    if symbols.lookup_var(name).is_none() {
                        return Err(todo!());
                    }
                }
                _ => {
                    return Err(todo!());
                }
            }

            analyze_stmt(functions, symbols, return_type, then_branch)?;
            if let Some(else_branch) = else_branch {
                analyze_stmt(functions, symbols, return_type, else_branch)?;
            }

            Ok(())
        }
        ASTStmt::While { condition, body } => {
            match condition {
                ASTExpr::IntLiteral(_) => {}
                ASTExpr::BinaryOp { op: _, lhs, rhs } => {
                    let lhs = analyze_expr(functions, symbols, lhs)?;
                    let rhs = analyze_expr(functions, symbols, rhs)?;
                    if lhs != rhs {
                        return Err(todo!());
                    }
                }
                ASTExpr::Var(name) => {
                    if symbols.lookup_var(name).is_none() {
                        return Err(todo!());
                    }
                }
                _ => {
                    return Err(todo!());
                }
            }

            analyze_stmt(functions, symbols, return_type, body)?;

            Ok(())
        }
    }
}

/// Analyzes an expression.
///
/// # Errors
/// Returns an error if the expression contains duplicate symbols or if a symbol is not defined.
pub fn analyze_expr(
    functions: &Functions,
    symbols: &mut Symbols,
    expr: &ASTExpr,
) -> CompileResult<ASTType> {
    match expr {
        ASTExpr::IntLiteral(_) => Ok(ASTType::Int),
        ASTExpr::Var(name) => {
            if let Some(ty) = symbols.lookup_var(name) {
                Ok(ty)
            } else {
                Err(todo!())
            }
        }
        ASTExpr::Call { name, args } => {
            let Some(func_signature) = functions.get(name) else {
                return Err(todo!());
            };

            if func_signature.param_types.len() != args.len() {
                return Err(todo!());
            }

            for (arg_expr, param_ty) in args.iter().zip(func_signature.param_types.iter()) {
                let arg_ty = analyze_expr(functions, symbols, arg_expr)?;
                if &arg_ty != param_ty {
                    return Err(todo!());
                }
            }

            Ok(func_signature.return_type)
        }
        ASTExpr::BinaryOp { op: _, lhs, rhs } => {
            let lhs = analyze_expr(functions, symbols, lhs)?;
            let rhs = analyze_expr(functions, symbols, rhs)?;

            if lhs != rhs {
                return Err(todo!());
            }
            // TODO

            Ok(lhs)
        }
        _ => Ok(ASTType::Void),
    }
}
