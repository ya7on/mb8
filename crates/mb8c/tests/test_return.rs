use mb8c::compile;

#[test]
fn test_return_binary_op() {
    let input = r"
    function foo(a: u8, b: u8): u8;
    begin
        return a + b;
    end
    ";
    compile(input).unwrap();
}

#[test]
fn test_return_type_mismatch() {
    let input = r"
    function foo(a: u8, b: u8): void;
    begin
        return a + b;
    end
    ";
    compile(input).unwrap_err();
}

#[test]
fn test_return_type_mismatc_void() {
    let input = r"
    function foo(a: u8, b: u8): u8;
    begin
        return;
    end
    ";
    compile(input).unwrap_err();
}
