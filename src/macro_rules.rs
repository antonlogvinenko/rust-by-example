fn intro() {
     macro_rules! cake {
          () => {
               42
          };
     }

     assert_eq!(cake!(), 42);
}

fn syntax() {
     //desognators: block, expr (expressions), ident (variable/function name), item, literal (literal constants), pat (pattern), path, stmt (statement), tt (token tree), ty (type), vis (visibility qualifier)

     macro_rules! create_function {
          ($func_name:ident) => {
               fn $func_name() {
                    println!("You called {}", stringify!($func_name))
               }
          };
     }

     create_function!(cake);
     cake();

     macro_rules! print_result {
          ($expression:expr) => {
               println!("{:?} = {:?}", stringify!($expression), $expression);
          };
     }

     print_result!(1 + 2);
}

pub fn main() {
     intro();
     syntax();
}
