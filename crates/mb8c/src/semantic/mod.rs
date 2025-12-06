use symbols::{FunctionSignature, Functions, Symbols};

use crate::{
    error::{CompileError, CompileResult},
    parser::ast::{Expr, Function, Program, Stmt, Type},
};

pub mod symbols;

/// Analyzes the given program.
///
/// # Errors
/// Returns an error if the program contains duplicate functions or if a function is not defined.
pub fn analyze(ast: &Program) -> CompileResult<()> {
    let mut functions = Functions::new();

    for function in &ast.functions {
        if functions.contains_key(&function.name) {
            return Err(CompileError::DuplicateFunction {
                name: function.name.clone(),
            });
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
pub fn analyze_function(functions: &Functions, input: &Function) -> CompileResult<()> {
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
    return_type: Type,
    stmt: &Stmt,
) -> CompileResult<()> {
    match stmt {
        Stmt::Block(stmts) => {
            symbols.enter_scope();
            for stmt in stmts {
                analyze_stmt(functions, symbols, return_type, stmt)?;
            }
            symbols.leave_scope();
            Ok(())
        }
        Stmt::Return(Some(expr)) => {
            let ty = analyze_expr(functions, symbols, expr)?;
            if ty != return_type {
                return Err(CompileError::TypeMismatch {
                    expected: return_type,
                    found: ty,
                });
            }
            Ok(())
        }
        Stmt::Return(None) => {
            if return_type == Type::Void {
                Ok(())
            } else {
                Err(CompileError::TypeMismatch {
                    expected: return_type,
                    found: Type::Void,
                })
            }
        }
        Stmt::Expression(expr) => {
            analyze_expr(functions, symbols, expr)?;
            Ok(())
        }
        Stmt::Declaration { name, ty, init: _ } => {
            symbols.insert(name.to_owned(), *ty)?;
            Ok(())
        }
        Stmt::If {
            condition,
            then_branch,
            else_branch,
        } => {
            match condition {
                Expr::IntLiteral(_) => {}
                Expr::BinaryOp { op: _, lhs, rhs } => {
                    let lhs = analyze_expr(functions, symbols, lhs)?;
                    let rhs = analyze_expr(functions, symbols, rhs)?;
                    if lhs != rhs {
                        return Err(CompileError::TypeMismatch {
                            expected: lhs,
                            found: rhs,
                        });
                    }
                }
                Expr::Var(name) => {
                    if symbols.lookup_var(name).is_none() {
                        return Err(CompileError::UndefinedSymbol { name: name.clone() });
                    }
                }
                _ => {
                    return Err(CompileError::TypeMismatch {
                        expected: return_type,
                        found: Type::Void,
                    });
                }
            }

            analyze_stmt(functions, symbols, return_type, then_branch)?;
            if let Some(else_branch) = else_branch {
                analyze_stmt(functions, symbols, return_type, else_branch)?;
            }

            Ok(())
        }
        Stmt::While { condition, body } => {
            match condition {
                Expr::IntLiteral(_) => {}
                Expr::BinaryOp { op: _, lhs, rhs } => {
                    let lhs = analyze_expr(functions, symbols, lhs)?;
                    let rhs = analyze_expr(functions, symbols, rhs)?;
                    if lhs != rhs {
                        return Err(CompileError::TypeMismatch {
                            expected: lhs,
                            found: rhs,
                        });
                    }
                }
                Expr::Var(name) => {
                    if symbols.lookup_var(name).is_none() {
                        return Err(CompileError::UndefinedSymbol { name: name.clone() });
                    }
                }
                _ => {
                    return Err(CompileError::TypeMismatch {
                        expected: return_type,
                        found: Type::Void,
                    });
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
    expr: &Expr,
) -> CompileResult<Type> {
    match expr {
        Expr::IntLiteral(_) => Ok(Type::Int),
        Expr::Var(name) => {
            if let Some(ty) = symbols.lookup_var(name) {
                Ok(ty)
            } else {
                Err(CompileError::UndefinedSymbol { name: name.clone() })
            }
        }
        Expr::Call { name, args } => {
            let Some(func_signature) = functions.get(name) else {
                return Err(CompileError::UndefinedSymbol { name: name.clone() });
            };

            if func_signature.param_types.len() != args.len() {
                return Err(CompileError::InvalidArgumentCount {
                    expected: func_signature.param_types.len(),
                    found: args.len(),
                });
            }

            for (arg_expr, param_ty) in args.iter().zip(func_signature.param_types.iter()) {
                let arg_ty = analyze_expr(functions, symbols, arg_expr)?;
                if &arg_ty != param_ty {
                    return Err(CompileError::TypeMismatch {
                        expected: *param_ty,
                        found: arg_ty,
                    });
                }
            }

            Ok(func_signature.return_type)
        }
        Expr::BinaryOp { op: _, lhs, rhs } => {
            let lhs = analyze_expr(functions, symbols, lhs)?;
            let rhs = analyze_expr(functions, symbols, rhs)?;

            if lhs != rhs {
                return Err(CompileError::TypeMismatch {
                    expected: lhs,
                    found: rhs,
                });
            }
            // TODO

            Ok(lhs)
        }
        _ => Ok(Type::Void),
    }
}
