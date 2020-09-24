mod printing;
mod primitives;
mod custom_types;

fn main() {
    println!("Hello, world!");
    printing::main();
    println!();

    primitives::main();
    println!();

    custom_types::main();
    println!();
}
