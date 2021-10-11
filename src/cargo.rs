/**
 * New project
 * cargo new foo
 * cargo new --lib foo
 *
 *
 *
 * Create two binaries:
 * foo
 * |--Cargo.toml
 * |--src
 *    |--main.rs
 *    |--bin
 *       |--my_other_bin.rs
 * for main.rs: cargo
 * for my_other_bin: cargo --bin my_other_bin
 *
 *
 *
 * Each file in ./test directory is a separate integration test
 * cargo test
 * cargo test test_name
 *
 *
 *
 * build.rs is run before build
 * or specify custom script with build = "build.rs" in Cargo.toml
 * build.rs is compiled an invoked prior to compiling everything else
 *
 *
 *
 * script print to output
 * is provided with inputs via environment variables
 * all lines starting with cargo: will be interpreted by cargo directly,
 * can be used to define parameters for the package's compilation
 */

pub fn main() {}
