use std::path::Path;

fn compile(input: &str) -> Result<Vec<u8>, asm::DiagnosticResult<Vec<u8>>> {
    let compilation = asm::compile_source(input, "<test>".to_string(), Path::new("."));
    match compilation {
        asm::DiagnosticResult {
            result: Some(result),
            ..
        } => Ok(result),
        compilation => Err(compilation),
    }
}

#[test]
fn nop() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("nop")?, vec![0x00, 0x00]);
    Ok(())
}

#[test]
fn halt() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("halt")?, vec![0x01, 0x00]);
    Ok(())
}

#[test]
fn halt_with_code() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("halt 0xab")?, vec![0x01, 0xab]);
    Ok(())
}

#[test]
fn sys() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("sys")?, vec![0x02, 0x00]);
    Ok(())
}

#[test]
fn mov() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("mov r0, r1")?, vec![0x10, 0x01]);
    Ok(())
}

#[test]
fn add() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("add r0, r1")?, vec![0x11, 0x01]);
    Ok(())
}

#[test]
fn sub() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("sub r0, r1")?, vec![0x12, 0x01]);
    Ok(())
}

#[test]
fn and() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("and r0, r1")?, vec![0x13, 0x01]);
    Ok(())
}

#[test]
fn or() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("or r0, r1")?, vec![0x14, 0x01]);
    Ok(())
}

#[test]
fn xor() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("xor r0, r1")?, vec![0x15, 0x01]);
    Ok(())
}

#[test]
fn shr() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("shr r0, r1")?, vec![0x16, 0x01]);
    Ok(())
}

#[test]
fn shl() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("shl r0, r1")?, vec![0x17, 0x01]);
    Ok(())
}

#[test]
fn cmp() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("cmp r0, r1")?, vec![0x18, 0x01]);
    Ok(())
}

#[test]
fn ldi() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("ldi r1, 0xab")?, vec![0x21, 0xab]);
    Ok(())
}

#[test]
fn jmp() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("jmp [r1:r2]")?, vec![0x30, 0x12]);
    Ok(())
}

#[test]
fn jr() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("jr 0xfe")?, vec![0x31, 0xfe]);
    Ok(())
}

#[test]
fn jzr() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("jzr 0x02")?, vec![0x32, 0x02]);
    Ok(())
}

#[test]
fn jnzr() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("jnzr 0x02")?, vec![0x33, 0x02]);
    Ok(())
}

#[test]
fn jcr() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("jcr 0x02")?, vec![0x34, 0x02]);
    Ok(())
}

#[test]
fn jncr() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("jncr 0x02")?, vec![0x35, 0x02]);
    Ok(())
}

#[test]
fn call() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("call [r1:r2]")?, vec![0x40, 0x12]);
    Ok(())
}

#[test]
fn ret() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("ret")?, vec![0x41, 0x00]);
    Ok(())
}

#[test]
fn push() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("push r1")?, vec![0x42, 0x10]);
    Ok(())
}

#[test]
fn pop() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("pop r1")?, vec![0x43, 0x10]);
    Ok(())
}

#[test]
fn ld() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("ld r0, [r1:r2]")?, vec![0x50, 0x12]);
    Ok(())
}

#[test]
fn st() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("st [r1:r2], r0")?, vec![0x60, 0x12]);
    Ok(())
}

#[test]
fn ldi16() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("ldi r1:r2, 0x1234")?, vec![0x21, 0x12, 0x22, 0x34]);
    Ok(())
}

#[test]
fn ldi16_label() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(
        compile("ldi r1:r2, target\ntarget:\n.data 0xab")?,
        vec![0x21, 0x00, 0x22, 0x04, 0xab]
    );
    Ok(())
}

#[test]
fn call_abs() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(
        compile("call [0x1234]")?,
        vec![0x29, 0x12, 0x2a, 0x34, 0x40, 0x9a]
    );
    Ok(())
}

#[test]
fn jmp_abs() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(
        compile("jmp [0x1234]")?,
        vec![0x29, 0x12, 0x2a, 0x34, 0x30, 0x9a]
    );
    Ok(())
}

#[test]
fn jr_label() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("jr [target]\ntarget:")?, vec![0x31, 0x00]);
    Ok(())
}

#[test]
fn jzr_label() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("jzr [target]\ntarget:")?, vec![0x32, 0x00]);
    Ok(())
}

#[test]
fn jnzr_label() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("jnzr [target]\ntarget:")?, vec![0x33, 0x00]);
    Ok(())
}

#[test]
fn jncr_label() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("jncr [target]\ntarget:")?, vec![0x35, 0x00]);
    Ok(())
}

#[test]
fn zero() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(compile("zero r1")?, vec![0x21, 0x00]);
    Ok(())
}

#[test]
fn inc() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(
        compile("inc r1")?,
        vec![0x42, 0x00, 0x20, 0x01, 0x11, 0x10, 0x43, 0x00]
    );
    Ok(())
}

#[test]
fn dec() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(
        compile("dec r1")?,
        vec![0x42, 0x00, 0x20, 0x01, 0x12, 0x10, 0x43, 0x00]
    );
    Ok(())
}

#[test]
fn inc16() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(
        compile("inc16 r1:r2")?,
        vec![
            0x42, 0x00, 0x20, 0xff, 0x12, 0x02, 0x43, 0x00, 0x32, 0x0a, 0x42, 0x00, 0x20, 0x01,
            0x11, 0x20, 0x43, 0x00, 0x31, 0x0a, 0x22, 0x00, 0x42, 0x00, 0x20, 0x01, 0x11, 0x10,
            0x43, 0x00, 0x00, 0x00,
        ]
    );
    Ok(())
}

#[test]
fn not() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(
        compile("not r1")?,
        vec![0x42, 0x00, 0x20, 0xff, 0x15, 0x10, 0x43, 0x00]
    );
    Ok(())
}

#[test]
fn cmp_immediate() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(
        compile("cmp r1, 0x42")?,
        vec![0x42, 0x00, 0x20, 0x42, 0x18, 0x10, 0x43, 0x00]
    );
    Ok(())
}

#[test]
fn shr_immediate() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(
        compile("shr r1, 0x04")?,
        vec![0x42, 0x00, 0x20, 0x04, 0x16, 0x10, 0x43, 0x00]
    );
    Ok(())
}

#[test]
fn shl_immediate() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(
        compile("shl r1, 0x04")?,
        vec![0x42, 0x00, 0x20, 0x04, 0x17, 0x10, 0x43, 0x00]
    );
    Ok(())
}

#[test]
fn swap() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(
        compile("swap r1, r2")?,
        vec![0x42, 0x10, 0x10, 0x12, 0x43, 0x20]
    );
    Ok(())
}

#[test]
fn mul() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(
        compile("mul r1, r2, r3")?,
        vec![
            0x21, 0x00, 0x42, 0x30, 0x11, 0x12, 0x42, 0x00, 0x20, 0x01, 0x12, 0x30, 0x43, 0x00,
            0x42, 0x00, 0x20, 0x00, 0x12, 0x03, 0x43, 0x00, 0x33, 0xec, 0x43, 0x30,
        ]
    );
    Ok(())
}

#[test]
fn st_abs() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(
        compile("st [0x1234], r1")?,
        vec![0x29, 0x12, 0x2a, 0x34, 0x61, 0x9a]
    );
    Ok(())
}

#[test]
fn st_abs_label() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(
        compile("st [target], r1\ntarget:\n.data 0xab")?,
        vec![0x29, 0x00, 0x2a, 0x06, 0x61, 0x9a, 0xab]
    );
    Ok(())
}

#[test]
fn ld_abs() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(
        compile("ld r1, [0x1234]")?,
        vec![0x29, 0x12, 0x2a, 0x34, 0x51, 0x9a]
    );
    Ok(())
}

#[test]
fn ld_abs_label() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(
        compile("ld r1, [target]\ntarget:\n.data 0xab")?,
        vec![0x29, 0x00, 0x2a, 0x06, 0x51, 0x9a, 0xab]
    );
    Ok(())
}

#[test]
fn ld_offset() -> Result<(), asm::DiagnosticResult<Vec<u8>>> {
    assert_eq!(
        compile("ld r1, [r2:r3 - 0x04]")?,
        vec![
            0x20, 0x04, 0x12, 0x30, 0x35, 0x08, 0x42, 0x00, 0x20, 0x01, 0x12, 0x20, 0x43, 0x00,
            0x51, 0x23,
        ]
    );
    Ok(())
}
