use mb8::vm::VirtualMachine;
use mb8_cli::filesystem::makefs;

use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_single_small_file() {
    let dir = tempdir().unwrap();
    //create file
    let file_path = dir.path().join("hello.txt");

    // write to file
    fs::write(&file_path, b"hello world").unwrap();

    let mut vm = VirtualMachine::default();
    makefs(vec![file_path.clone()], &mut vm);

    //Preparing disk block
    let used_flag = vm.devices.read(0xF200 + 0);
    let start_block = vm.devices.read(0xF200 + 1);
    let size_blocks = vm.devices.read(0xF200 + 2);

    //comparing disk block
    assert_eq!(used_flag, 1);
    assert_eq!(start_block, 1);
    assert_eq!(size_blocks, 1);

    //read file name without extension
    let name_bytes: Vec<u8> = (0..5).map(|i| vm.devices.read(0xF200 + 3 + i)).collect();

    assert_eq!(&name_bytes, b"hello");

    //data block starts at 1 with offset of 256
    let first_data_byte = vm.devices.read(0xF200 + 256);
    assert_eq!(first_data_byte, b'h');
}

#[test]
fn test_two_files() {
    let dir = tempdir().unwrap();
    let a = dir.path().join("a.bin");
    let b = dir.path().join("b.bin");

    fs::write(&a, b"AAAA").unwrap();
    fs::write(&b, b"BBBBBBBB").unwrap();

    let mut vm = VirtualMachine::default();
    makefs(vec![a.clone(), b.clone()], &mut vm);

    //Comparing disk block file a
    assert_eq!(vm.devices.read(0xF200 + 0), 1);
    assert_eq!(vm.devices.read(0xF200 + 1), 1);
    assert_eq!(vm.devices.read(0xF200 + 2), 1);

    //Comparing disk block file b
    assert_eq!(vm.devices.read(0xF200 + 16), 1);
    assert_eq!(vm.devices.read(0xF200 + 17), 2);
    assert_eq!(vm.devices.read(0xF200 + 18), 1);
}
