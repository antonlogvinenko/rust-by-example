mod conversion;
mod custom_types;
mod primitives;
mod printing;
mod variable_binding;

//get rid of println!s in primitives/printing - replace with assert_eq!
//primitives: 2 + 5
//custom types: 3 + 6
//move variable binding after that

//todo make code runnable?
//todo better comments?

fn main() {
    printing::main();
    primitives::main();
    custom_types::main();
    variable_binding::main();
    conversion::main();
}
