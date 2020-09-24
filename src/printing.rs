/**
 * Different macors for printing
 * to string/io::stdio/io::stderr
 * with or without newline
 */
fn different_prints() {
    //format creates formatted String
    let value = "value";
    let formatted = format!("Some formatted string with {}", value);
    println!("{}", formatted);

    //print! prints formatted String to console
    print!("Print {} without EOL", value);
    println!();

    //println! prints formatted String to io::stdio with EOL
    println!("Print some {} with EOL", value);

    //print to io::stderr
    eprint!("Print some {} to stderr without EOL", value);
    println!();

    //print to io::stderr with EOL
    eprintln!("Print some {} to stderr with EOL", value);
    println!();
}

fn print_with_positional_args() {
    //Numbered arguments
    println!(
        "Can print arg {0}, then {1}, and then {2}",
        "arg0", "arg1", "arg2"
    );
    //Named arguments
    println!(
        "Can print arg {arg0}, then {arg1}, and then {arg2}",
        arg0 = "arg0",
        arg1 = "arg1",
        arg2 = "arg2"
    );
    println!();
}

fn print_with_special_formatting() {
    println!("Printing binary {:b}", 234);
    println!(
        "Printing with padding {value:>witdh$}",
        value = "value",
        witdh = 10
    );
    println!(
        "Printing with filled padding {value:>0witdh$}",
        value = 4,
        witdh = 20
    );

    println!();
}

fn debug_trait() {
    #[derive(Debug)]
    struct DebugPrintable(i32);

    println!(
        "Debug print for {0:?} and {1:?}",
        "string value",
        DebugPrintable(42)
    );
    println!(
        "Debug pretty print for {0:#?} and {1:#?}",
        "string value",
        DebugPrintable(42)
    );
    println!();
}

fn display_trait() {
    use std::fmt;

    //fmt::Result is a type alias
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let vec = &self.0;

            write!(f, "[")?;

            for (count, v) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", v)?;
            }

            write!(f, "]")
        }
    }

    println!("Custom implemented display: {}", List(vec![1, 2]));

    println!();
}

fn formatting() {
    println!("Format as binary {:b}", 234);
    println!("Format as hex {:X}", 234);
    println!("Format as oct {:o}", 234);
    println!("Format as oct {:.2}", 234.4567);
}

pub fn main() {
    different_prints();
    println!();

    print_with_positional_args();
    println!();

    print_with_special_formatting();
    println!();

    debug_trait();
    println!();

    display_trait();
    println!();

    formatting();
    println!();
}
