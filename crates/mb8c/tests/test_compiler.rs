use mb8c::compile;

#[test]
fn test_main() {
    let src = include_str!("../examples/sum.c");
    compile(&src).unwrap();
}
