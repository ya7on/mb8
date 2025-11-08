mod ops;
mod parser;
mod registers;
mod vm;

const DEFAULT_PROGRAM: &[u8] = &[0x00, 0x00, 0x01, 0x00];

fn main() {
    let mut vm = vm::VirtualMachine::new();
    vm.load_memory(DEFAULT_PROGRAM);
    vm.run();
}
