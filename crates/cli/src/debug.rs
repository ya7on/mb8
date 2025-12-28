//write a debug helper that can be used by both architectures.
use std::io;
use std::io::Write;

pub fn debug_shell() {
    println!(" MB8 Debugger");

    loop {
        io::stdout().flush().unwrap();

        let mut input_string = String::new();
        io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to readline");

        let trimmed_input = input_string.trim();

        println!("you entered {} into the debug terminal", trimmed_input);
    }
}

pub fn execute_next_instruction() {
    //execute the next instruction
}

pub fn continue_normal_execution() {}

pub fn print_registers_and_flags() {}

pub fn print_memory() {}

pub fn print_help() {}
