mod custom_types;
mod primitives;
mod printing;
mod variable_binding;
mod types;

extern crate termion;
use termion::{color, style};

fn main() {
    println!("Soimething {}Red adasd{}6666", color::Fg(color::Red), style::Reset);

    printing::main();
    println!();

    primitives::main();
    println!();

    custom_types::main();
    println!();

    variable_binding::main();
    println!();

    types::main();
    println!();
}
