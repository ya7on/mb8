use mb8c::{parser::base::Parser, semantic::analyze, tokenizer::lexer::Lexer};

#[test]
fn test_main() {
    let src = r#"
        int func(int a) {
            return a;
        }

        int main(int a, int b) {
            func(1);
            return 1;
        }
    "#;
    let tokens = Lexer::new(src).tokenize().unwrap();
    let ast = Parser::new(tokens).parse_program().unwrap();
    analyze(&ast).unwrap();
}
