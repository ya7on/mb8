use mb8c::{
    error::CompileError,
    parser::{ast::Type, base::Parser},
    semantic::analyze,
    tokenizer::lexer::Lexer,
};

#[test]
fn test_return_type() {
    // Expected void, found int
    {
        let src = r#"
        void main() {
            return 1;
        }
        "#;
        let tokens = Lexer::new(src).tokenize().unwrap();
        let ast = Parser::new(tokens).parse_program().unwrap();
        assert_eq!(
            analyze(&ast).unwrap_err(),
            CompileError::TypeMismatch {
                expected: Type::Void,
                found: Type::Int
            }
        );
    }

    // Expected void, found char
    {
        let src = r#"
        void main() {
            char a = 1;
            return a;
        }
        "#;
        let tokens = Lexer::new(src).tokenize().unwrap();
        let ast = Parser::new(tokens).parse_program().unwrap();
        assert_eq!(
            analyze(&ast).unwrap_err(),
            CompileError::TypeMismatch {
                expected: Type::Void,
                found: Type::Char
            }
        );
    }

    // Expected void, found void
    {
        let src = r#"
        void main() {
            return;
        }
        "#;
        let tokens = Lexer::new(src).tokenize().unwrap();
        let ast = Parser::new(tokens).parse_program().unwrap();
        let result = analyze(&ast);
        assert!(result.is_ok(), "{result:?}");
    }

    // Expected int, found void
    {
        let src = r#"
        int main() {
            return;
        }
        "#;
        let tokens = Lexer::new(src).tokenize().unwrap();
        let ast = Parser::new(tokens).parse_program().unwrap();
        assert_eq!(
            analyze(&ast).unwrap_err(),
            CompileError::TypeMismatch {
                expected: Type::Int,
                found: Type::Void
            }
        );
    }

    // Expected int, found char
    {
        let src = r#"
        int main() {
            char a = 1;
            return a;
        }
        "#;
        let tokens = Lexer::new(src).tokenize().unwrap();
        let ast = Parser::new(tokens).parse_program().unwrap();
        assert_eq!(
            analyze(&ast).unwrap_err(),
            CompileError::TypeMismatch {
                expected: Type::Int,
                found: Type::Char
            }
        );
    }

    // Expected int, found int
    {
        let src = r#"
        int main() {
            return 0;
        }
        "#;
        let tokens = Lexer::new(src).tokenize().unwrap();
        let ast = Parser::new(tokens).parse_program().unwrap();
        let result = analyze(&ast);
        assert!(result.is_ok(), "{result:?}");
    }
}

#[test]
fn test_call() {
    // Call a function
    {
        let src = r#"
        int foo() {
            return 0;
        }

        int main() {
            foo();
        }
        "#;
        let tokens = Lexer::new(src).tokenize().unwrap();
        let ast = Parser::new(tokens).parse_program().unwrap();
        let result = analyze(&ast);
        assert!(result.is_ok(), "{result:?}");
    }

    // Call a function with arguments
    {
        let src = r#"
        int foo(int a) {
            return a;
        }

        int main() {
            foo(1);
        }
        "#;
        let tokens = Lexer::new(src).tokenize().unwrap();
        let ast = Parser::new(tokens).parse_program().unwrap();
        let result = analyze(&ast);
        assert!(result.is_ok(), "{result:?}");
    }

    // Call a function with arguments, wrong argument count
    {
        let src = r#"
        int foo(int a) {
            return a;
        }

        int main() {
            foo(1, 2);
        }
        "#;
        let tokens = Lexer::new(src).tokenize().unwrap();
        let ast = Parser::new(tokens).parse_program().unwrap();
        let result = analyze(&ast);
        assert_eq!(
            result.unwrap_err(),
            CompileError::InvalidArgumentCount {
                expected: 1,
                found: 2
            }
        );
    }

    // Call a function with arguments, wrong argument count
    {
        let src = r#"
        int foo(int a) {
            return a;
        }

        int main() {
            foo();
        }
        "#;
        let tokens = Lexer::new(src).tokenize().unwrap();
        let ast = Parser::new(tokens).parse_program().unwrap();
        let result = analyze(&ast);
        assert_eq!(
            result.unwrap_err(),
            CompileError::InvalidArgumentCount {
                expected: 1,
                found: 0
            }
        );
    }
}

#[test]
fn test_if() {
    {
        let src = r#"
        int main() {
            if (1) {
                return 1;
            }
        }
        "#;
        let tokens = Lexer::new(src).tokenize().unwrap();
        let ast = Parser::new(tokens).parse_program().unwrap();
        let result = analyze(&ast);
        assert!(result.is_ok(), "{result:?}");
    }

    {
        let src = r#"
        int main() {
            int a = 1;
            if (a) {
                return 1;
            }
        }
        "#;
        let tokens = Lexer::new(src).tokenize().unwrap();
        let ast = Parser::new(tokens).parse_program().unwrap();
        let result = analyze(&ast);
        assert!(result.is_ok(), "{result:?}");
    }

    {
        let src = r#"
        int main() {
            if (a) {
                return 1;
            }
        }
        "#;
        let tokens = Lexer::new(src).tokenize().unwrap();
        let ast = Parser::new(tokens).parse_program().unwrap();
        let result = analyze(&ast);
        assert_eq!(
            result.unwrap_err(),
            CompileError::UndefinedSymbol {
                name: "a".to_string()
            }
        );
    }
}
