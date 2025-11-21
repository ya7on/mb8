use mb8::vm::VirtualMachine;

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
        assert_eq!(vm.devices.read(0x0150 + i), i as u8, "{:?}", i);
    }
}
