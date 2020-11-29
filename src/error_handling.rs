fn panic() {
     //use env var RUST_BACKTRACE=0|1|full
     //panic!("bla");
}

fn option() {
     //Option: Some(T) | None
     let x: Option<i32> = Some(42);
     let y: Option<i32> = None;

     //unsafe: .unwrap => panic!
     // y.unwrap();
     assert_eq!(x.unwrap(), 42);

     //safe: pattern matching or ? operator
     fn increase1(x: Option<i32>) -> Option<i32> {
          let value = x?;
          Some(value + 1)
     }
     fn increase2(x: Option<i32>) -> Option<i32> {
          match x {
               Some(value) => Some(value + 1),
               None => None,
          }
     }
     assert_eq!(increase1(x), Some(43));
     assert_eq!(increase1(y), None);
     assert_eq!(increase2(x), Some(43));
     assert_eq!(increase2(y), None);

     //combining with map and map_then
     let x2 = x.map(|v| v + 1);
     let x3 = x.and_then(|v| Some(v));
     assert_eq!(x2, Some(43));
     assert_eq!(x3, Some(42));
}

fn result() {
     use std::error;
     use std::fmt;

     #[derive(Debug, Clone, PartialEq, Copy)]
     struct Error;

     impl fmt::Display for Error {
          fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
               write!(f, "invalid first item to double")
          }
     }

     impl error::Error for Error {};

     //Result: Ok(T) | Err(E)
     let x: Result<u32, Error> = Ok(42);
     let y: Result<u32, Error> = Err(Error);

     //unsafe: .unwrap => panic!
     assert_eq!(x, Ok(42));
     //assert_eq!(y.unwrap(), unimplemented!());

     //safe: pattern matching or ? operator
     fn increase1(x: Result<u32, Error>) -> Result<u32, Error> {
          match x {
               Ok(value) => Ok(value + 1),
               Err(err) => Err(err),
          }
     }
     fn increase2(x: Result<u32, Error>) -> Result<u32, Error> {
          let value = x?;
          Ok(value + 1)
     }
     assert_eq!(increase1(x), Ok(43));
     // assert_eq!(increase1(y).unwrap(), 42);
     assert_eq!(increase2(x), Ok(43));
     // assert_eq!(increase2(y), Ok(42));

     //combining with map and map_then
     let x2 = x.map(|v| v + 1);
     let y2 = y.map(|v| v + 1);
     assert_eq!(x2, Ok(43));
     assert_eq!(y2, Err(Error));
     let x3 = x.and_then(|v| Ok(v + 1));
     let y3 = y.and_then(|v| Ok(v + 1));
     assert_eq!(x3, Ok(43));
     assert_eq!(y3, Err(Error));

     //using Result in main
     // fn main() -> Result<(), Error>

     //type aliases
     // use std::io::Result;
     // type AliasedResult<T> = Result<T, Error>;

     //legacy: try? macro
     //try?(expr) == expr?
}

fn multiple_error_types() {
     //converting Option to Result
     //option.ok_or
     //option.map_or

     //replacing errors with specific error type
     //option.ok_or
     //result.map_err

     //boxing errors
     //error.into()

     //auto boxing errors with ?

     //wrapping errors
}

pub fn main() {
     panic();
     option();
     result();
     multiple_error_types();
}
