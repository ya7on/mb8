use mb8::vm::VirtualMachine;
use mb8_cli::filesystem::makefs;
use std::fs;
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

    let disk_img = vm.devices.disk().dump();

    let used_flag = disk_img[0];
    let start_block = disk_img[1];
    let size_blocks = disk_img[2];

    //comparing disk block
    assert_eq!(used_flag, 1);
    assert_eq!(start_block, 1);
    assert_eq!(size_blocks, 1);

    //read file name without extension
    let name_bytes: Vec<u8> = disk_img[3..8].to_vec();
    assert_eq!(&name_bytes, b"hello");

    //data block starts at 1 with offset of 256
    assert_eq!(disk_img[256], b'h');
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

    let disk_img = vm.devices.disk().dump();

    let used_flag = disk_img[0];
    let start_block = disk_img[1];
    let size_blocks = disk_img[2];

    let used_flag_b = disk_img[16];
    let start_block_b = disk_img[17];
    let size_blocks_b = disk_img[18];

    //Comparing disk block file a
    assert_eq!(used_flag, 1);
    assert_eq!(start_block, 1);
    assert_eq!(size_blocks, 1);

    //Comparing disk block file b
    assert_eq!(used_flag_b, 1);
    assert_eq!(start_block_b, 2);
    assert_eq!(size_blocks_b, 1);
}
