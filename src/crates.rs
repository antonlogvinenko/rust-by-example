/*
 * crate == compilation unit; modules don't get compiled, crates do
 *
 * "rustc some_file.rs"
 * - some_file.rs gets treated as crate
 * - all mod declarations are substituted with mod contents
 * - crate gets compiled
 *
 * A crate can be compiled in library or executable
 * - binary by default
 * - lib if --crate-type=lib is passed to rustc
 */

pub fn main() {
    extern crate termion;

    println!(
        "{}Blue{}",
        termion::color::Fg(termion::color::Red),
        termion::color::Fg(termion::color::Reset)
    );

    use termion::color;
    println!("{}Blue{}", color::Fg(color::Red), color::Fg(color::Reset));

    use termion::color as c;
    println!("{}Blue{}", c::Fg(c::Red), c::Fg(c::Reset));
}
