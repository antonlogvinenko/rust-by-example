mod custom_types;
mod primitives;
mod printing;
mod variable_binding;
mod expressions;
mod flow_of_control;
mod functions;
mod modules;
mod crates;
mod cargo;
mod attributes;
mod generics;
mod scoping_rules;
mod traits;

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
}
