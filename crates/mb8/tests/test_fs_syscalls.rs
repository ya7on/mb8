use mb8::vm::VirtualMachine;
use mb8_isa::registers::Register;

#[test]
fn test_sys_fs_list() {
    let bin = include_bytes!("../../../kernel/tests/test_sys_fs_list.bin");
    let mut vm = VirtualMachine::default();
    let mut img = vec![0; 65536].into_boxed_slice();
    for i in 0..256 {
        img[i] = i as u8;
    }
    vm.devices.disk().set(img.try_into().unwrap());
    vm.load_rom(bin);
    vm.run();

    for i in 0..256 {
        assert_eq!(vm.devices.read(0x0150 + i), i as u8, "{i:?}");
    }
}

#[test]
fn test_sys_fs_find() {
    let bin = include_bytes!("../../../kernel/tests/test_sys_fs_find.bin");
    let mut img = vec![0; 65536].into_boxed_slice();
    img[0] = 1; // status
    img[1] = 2; // start block
    img[2] = 1; // size
    img[3..8].copy_from_slice(b"file\0");

    let mut vm = VirtualMachine::default();
    vm.devices.disk().set(img.try_into().unwrap());
    vm.load_rom(bin);
    vm.run();

    assert_eq!(vm.registers.read(Register::R0), 0);
    assert_eq!(vm.registers.read(Register::R1), 2);
    assert_eq!(vm.registers.read(Register::R2), 1);
}

#[test]
fn test_sys_fs_find_not_exist() {
    let bin = include_bytes!("../../../kernel/tests/test_sys_fs_find.bin");
    let mut img = vec![0; 65536].into_boxed_slice();
    img[0] = 1; // status
    img[1] = 2; // start block
    img[2] = 1; // size
    img[3..8].copy_from_slice(b"ffff\0");

    let mut vm = VirtualMachine::default();
    vm.devices.disk().set(img.try_into().unwrap());
    vm.load_rom(bin);
    vm.run();

    assert_eq!(vm.registers.read(Register::R0), 1);
}

#[test]
fn test_sys_fs_read() {
    let bin = include_bytes!("../../../kernel/tests/test_sys_fs_read.bin");
    let mut img = vec![0; 65536].into_boxed_slice();
    img[0] = 1; // status
    img[1] = 2; // start block
    img[2] = 2; // size
    img[3..8].copy_from_slice(b"file\0");
    img[256 * 2..256 * 4].copy_from_slice(&[1; 256 * 2]);

    let mut vm = VirtualMachine::default();
    vm.devices.disk().set(img.try_into().unwrap());
    vm.load_rom(bin);
    vm.run();

    assert_eq!(vm.registers.read(Register::R0), 0);

    for i in 0..256 * 2 {
        assert_eq!(vm.devices.read(i), 1, "{i:?}");
    }
}

#[test]
#[ignore = "TODO"]
fn test_sys_fs_write() {
    todo!()
}

#[test]
#[ignore = "TODO"]
fn test_sys_fs_delete() {
    todo!()
}
