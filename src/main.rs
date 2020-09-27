mod custom_types;
mod primitives;
mod printing;
mod variable_binding;
mod expressions;

//get rid of println!s in primitives/printing - replace with assert_eq!
//extract primitive types conversion

//todo make code runnable?
//todo better comments?

fn main() {
    printing::main();
    primitives::main();
    custom_types::main();
    variable_binding::main();
    expressions::main();
}
