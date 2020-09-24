mod custom_types;
mod primitives;
mod printing;

fn main() {
    printing::main();
    println!();

    primitives::main();
    println!();

    custom_types::main();
    println!();
}
