mod mem;
mod ops;
mod registers;
mod vm;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = &args[1];
    let source = std::fs::read(input_path).unwrap_or_default();

    let mut vm = vm::VirtualMachine::new();

    vm.load_rom(&source);
    vm.run();
}
