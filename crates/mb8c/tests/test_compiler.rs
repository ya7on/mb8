use mb8c::compile;

#[test]
fn test_main() {
    let src = include_str!("../examples/sum.c");
    compile(&src).unwrap();

    let src = include_str!("../examples/cond.c");
    compile(&src).unwrap();

    let src = include_str!("../examples/loop.c");
    compile(&src).unwrap();
}
