use mb8c::compile;

#[test]
fn test_var_assign() {
    let input = r"
    function foo(a: u8, b: u8): void;
    var
        c: u8;
    begin
        c = (a - 2) + (b + 2);
    end
    ";
    compile(input).unwrap();
}

#[test]
fn test_unknown_var() {
    let input = r"
    function foo(a: u8, b: u8): void;
    begin
        c = a + b;
    end
    ";
    compile(input).unwrap_err();
}
