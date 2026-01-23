use mb8c::compile;
use mb8c::error::CompileError;

#[test]
fn test_pointer_roundtrip() {
    let input = r"
    function foo(a: u8): u8;
    var
        ptr: *u8;
    begin
        ptr = &a;
        return *ptr;
    end
    ";

    compile(input).unwrap();
}

#[test]
fn test_deref_non_pointer() {
    let input = r"
    function foo(a: u8): u8;
    begin
        return *a;
    end
    ";

    let err = compile(input).unwrap_err();
    assert!(matches!(
        err.first(),
        Some(CompileError::ExpectedPointer { .. })
    ));
}
