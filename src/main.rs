mod attributes;
mod cargo;
mod crates;
mod custom_types;
mod error_handling;
mod expressions;
mod flow_of_control;
mod functions;
mod generics;
mod macro_rules;
mod modules;
mod primitives;
mod printing;
mod scoping_rules;
mod std_library_types;
mod traits;
mod variable_binding;

//todo make code runnable?
//todo better comments?

extern crate termion;

fn main() {
    printing::main();
    primitives::main();
    custom_types::main();
    variable_binding::main();
    expressions::main();
    flow_of_control::main();
    functions::main();
    modules::main();
    crates::main();
    cargo::main();
    attributes::main();
    generics::main();
    scoping_rules::main();
    traits::main();
    macro_rules::main();
    error_handling::main();
    std_library_types::main();
}
