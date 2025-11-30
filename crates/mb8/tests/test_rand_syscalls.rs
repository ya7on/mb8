use mb8::{dev::Device, vm::VirtualMachine};

#[test]
fn test_sys_rand_deterministic() {
    let bin = include_bytes!("../../../kernel/tests/test_sys_rand.bin");

    let mut vm1 = VirtualMachine::default();
    vm1.devices.rand().seed(234);
    vm1.load_rom(bin);
    vm1.run();

    let mut out1 = [0u8; 16];
    for i in 0..16 {
        out1[i] = vm1.devices.read(0x0200 + i as u16);
    }

    let mut vm2 = VirtualMachine::default();
    vm2.devices.rand().seed(234);
    vm2.load_rom(bin);
    vm2.run();

    let mut out2 = [0u8; 16];
    for i in 0..16 {
        out2[i] = vm2.devices.read(0x0200 + i as u16);
    }

    assert_eq!(out1, out2);

    let rng_value = vm1.devices.rand().read(0);
    assert_ne!(out1[0], rng_value);
}
