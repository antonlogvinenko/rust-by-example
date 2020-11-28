fn intro() {
     macro_rules! cake {
          () => {
               42
          };
     }

     assert_eq!(cake!(), 42);
}

fn syntax_designators() {
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

fn syntax_overload() {
     macro_rules! test {
          ($left:expr, and $right:expr) => {
               println!(
                    "{:?} and {:?} is {:?}",
                    stringify!($left),
                    stringify!($right),
                    $left && $right
               )
          };
          ($left:expr; or $right:expr) => {
               println!(
                    "{:?} or {:?} is {:?}",
                    stringify!($left),
                    stringify!($right),
                    $left || $right
               )
          };
     }

     test!(1i32 + 1 == 2i32, and 2i32 * 2 == 4i32);
     test!(true; or false);
}

fn syntax_repeat() {
     macro_rules! find_min {
          // Base case:
          ($x:expr) => ($x);
          // `$x` followed by at least one `$y,`
          ($x:expr, $($y:expr),+) => (
              // Call `find_min!` on the tail `$y`
              std::cmp::min($x, find_min!($($y),+))
          )
      }

      assert_eq!(find_min!(1u32), 1);
      assert_eq!(find_min!(1u32 + 2, 2u32), 2);
      assert_eq!(find_min!(5u32, 2u32 * 3, 4u32), 4);
}

pub fn main() {
     intro();
     syntax_designators();
     syntax_overload();
     syntax_repeat();
}
