mod custom_types;
mod primitives;
mod printing;
mod variable_binding;
mod expressions;
mod flow_of_control;
mod functions;

//todo make code runnable?
//todo better comments?

fn main() {
    printing::main();
    primitives::main();
    custom_types::main();
    variable_binding::main();
    expressions::main();
    flow_of_control::main();
    functions::main();
}
